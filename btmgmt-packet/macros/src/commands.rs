use proc_macro2::TokenStream;
use syn::{Expr, Ident, Item, ItemMod, Token, parse_quote};
use syn::parse::{Parse, ParseStream};
use quote::ToTokens;

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
            Ok(Self {
                code,
                reply,
            })
        } else {
            Err(input.error("no name or code found."))
        }
    }
}

struct Target(Ident, CommandAttr);

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
}

fn collect_targets(items: &mut Vec<Item>) -> syn::Result<Vec<Target>> {
    let mut result = vec![];

    for item in items {
        if let Item::Struct(item) = item {
            let mut newattrs = vec![];
            for attr in item.attrs.drain(..) {
                if attr.path.is_ident("command") {
                    let attr = attr.parse_args::<CommandAttr>()?;
                    result.push(Target(item.ident.clone(), attr));
                } else {
                    newattrs.push(attr);
                }
            }
            item.attrs = newattrs;
        }
    }

    Ok(result)
}

fn apply(attr: Args, item: &mut ItemMod) -> syn::Result<()> {
    let mut contents = if let Some((_, contents)) = &mut item.content {
        contents
    } else {
        return Err(syn::Error::new_spanned(item, "no mod body found."));
    };

    let name = &attr.name;
    let trait_ = &attr.trait_;
    let codes = &attr.codes;
    let targets = collect_targets(&mut contents)?;

    if targets.is_empty() {
        return Err(syn::Error::new_spanned(item, "no target (`#[command(..)]`) found."));
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

    contents.push(parse_quote! {
        pub trait #trait_ {
            const CODE: #codes;
            type Reply;
        }
    });

    contents.push(parse_quote! {
        #[derive(Debug)]
        pub enum #name {
            #( #idents(#idents), )*
        }
    });

    contents.push(parse_quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, ::btmgmt_packet::pack::Pack)]
        #[pack(u16)]
        pub enum #codes {
            #( #idents = #vals, )*
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
