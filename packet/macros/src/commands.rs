use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Attribute, Expr, Ident, Item, ItemMod, Token};

#[derive(Debug)]
struct Args {
    name: Ident,
    codes: Ident,
    trait_: Ident,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut codes = None;
        let mut trait_ = None;

        while input.peek(Ident) || input.peek(Token![trait]) {
            if input.peek(Ident) {
                let k = input.parse::<Ident>()?;
                match k.to_string().as_str() {
                    "name" => {
                        input.parse::<Token![=]>()?;
                        name = Some(input.parse()?);
                    }
                    "codes" => {
                        input.parse::<Token![=]>()?;
                        codes = Some(input.parse()?);
                    }
                    unknown => return Err(input.error(format!("unknown name {}", unknown))),
                }
            } else if input.peek(Token![trait]) {
                input.parse::<Token![trait]>()?;
                input.parse::<Token![=]>()?;
                trait_ = Some(input.parse::<Ident>()?);
            } else {
                return Err(input.error(""));
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        if let (Some(name), Some(codes), Some(trait_)) = (name, codes, trait_) {
            Ok(Self {
                name,
                codes,
                trait_,
            })
        } else {
            Err(input.error("no name, code or trait found."))
        }
    }
}

struct CommandAttr {
    code: Expr,
    reply: Ident,
}

impl Parse for CommandAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut code = None;
        let mut reply = None;

        while input.peek(syn::Ident) {
            let ident = input.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "code" => {
                    input.parse::<Token![=]>()?;
                    code = Some(input.parse::<Expr>()?);
                }
                "reply" => {
                    input.parse::<Token![=]>()?;
                    reply = Some(input.parse::<Ident>()?);
                }
                other => return Err(input.error(format!("unknown name {}", other))),
            };

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        if let (Some(code), Some(reply)) = (code, reply) {
            Ok(Self { code, reply })
        } else {
            Err(input.error("no name or code found."))
        }
    }
}

struct Target(Ident, CommandAttr, Vec<Attribute>);

impl Target {
    fn ident(&self) -> &Ident {
        &self.0
    }

    fn val(&self) -> &Expr {
        &self.1.code
    }

    fn reply(&self) -> &Ident {
        &self.1.reply
    }

    fn docs(&self) -> &[Attribute] {
        &self.2
    }
}

fn collect_targets(items: &mut Vec<Item>) -> syn::Result<Vec<Target>> {
    let mut result = vec![];

    for item in items {
        if let Item::Struct(item) = item {
            let mut newattrs = vec![];
            let mut command_attr = None;
            for attr in item.attrs.drain(..) {
                if attr.path.is_ident("command") {
                    command_attr = Some(attr.parse_args::<CommandAttr>()?);
                } else {
                    newattrs.push(attr);
                }
            }
            item.attrs = newattrs;
            if let Some(attr) = command_attr {
                let docs = item
                    .attrs
                    .iter()
                    .filter(|a| a.path.is_ident("doc"))
                    .cloned()
                    .collect::<Vec<_>>();
                result.push(Target(item.ident.clone(), attr, docs));
            }
        }
    }

    Ok(result)
}

fn apply(attr: Args, item: &mut ItemMod) -> syn::Result<()> {
    let docs = item
        .attrs
        .iter()
        .filter(|a| a.path.is_ident("doc"))
        .cloned()
        .collect::<Vec<_>>();

    let contents = if let Some((_, contents)) = &mut item.content {
        contents
    } else {
        return Err(syn::Error::new_spanned(item, "no mod body found."));
    };

    let name = &attr.name;
    let trait_ = &attr.trait_;
    let codes = &attr.codes;
    let targets = collect_targets(contents)?;

    if targets.is_empty() {
        return Err(syn::Error::new_spanned(
            item,
            "no target (`#[command(..)]`) found.",
        ));
    }

    for target in &targets {
        let ident = target.ident();
        let reply = target.reply();

        contents.push(parse_quote! {
            impl #trait_ for #ident {
                const CODE: #codes = #codes::#ident;
                type Reply = #reply;
            }
        });

        contents.push(parse_quote! {
            impl ::std::convert::From<#ident> for #name {
                fn from(v: #ident) -> Self {
                    Self::#ident(v)
                }
            }
        });
    }

    let idents = targets.iter().map(Target::ident).collect::<Vec<_>>();
    let vals = targets.iter().map(Target::val).collect::<Vec<_>>();
    let tdocs = targets.iter().map(Target::docs).collect::<Vec<_>>();

    contents.push(parse_quote! {
        /// Represents a management api command.
        pub trait #trait_: ::std::convert::Into<#name> {
            /// Command code.
            const CODE: #codes;
            /// Return type for this command.
            type Reply: ::btmgmt_packet_helper::pack::Unpack;
        }
    });

    contents.push(parse_quote! {
        #( #docs )*
        /// This struct internal use only.
        #[derive(Debug)]
        pub enum #name {
            #( #( #tdocs )* #idents(#idents), )*
        }
    });

    contents.push(parse_quote! {
        impl #name {
            #[doc(hidden)]
            pub fn code(&self) -> #codes {
                match self {
                    #( Self::#idents(..) => #codes::#idents, )*
                }
            }

            #[doc(hidden)]
            pub fn pack_inner<W>(&self, write: &mut W) -> ::btmgmt_packet_helper::pack::Result<()> where W: ::std::io::Write {
                match self {
                    #( Self::#idents(inner) => inner.pack(write), )*
                }
            }
        }
    });

    contents.push(parse_quote! {
        /// Command Code
        #[derive(Debug, Clone, PartialEq, Eq, Hash, ::btmgmt_packet_helper::pack::Pack, ::btmgmt_packet_helper::pack::Unpack)]
        #[pack(u16)]
        pub enum #codes {
            #( #( #tdocs )* #idents = #vals, )*
        }
    });

    Ok(())
}

pub fn commands(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = match syn::parse2::<ItemMod>(input) {
        Ok(item) => item,
        Err(err) => return err.to_compile_error(),
    };

    let attr = match syn::parse2::<Args>(attr) {
        Ok(attr) => attr,
        Err(err) => return err.to_compile_error(),
    };

    if let Err(err) = apply(attr, &mut item) {
        return err.to_compile_error();
    };

    item.to_token_stream()
}
