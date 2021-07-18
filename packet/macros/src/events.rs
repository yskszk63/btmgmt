use proc_macro2::TokenStream;
use syn::{Attribute, Expr, Ident, Item, ItemMod, Token, parse_quote};
use syn::parse::{Parse, ParseStream};
use quote::ToTokens;

#[derive(Debug)]
struct Args {
    name: Ident,
    codes: Ident,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut codes = None;

        while input.peek(syn::Ident) {
            let ident = input.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "name" => {
                    input.parse::<Token![=]>()?;
                    name = Some(input.parse::<Ident>()?);
                }
                "codes" => {
                    input.parse::<Token![=]>()?;
                    codes = Some(input.parse::<Ident>()?);
                }
                other => return Err(input.error(format!("unknown name {}", other))),
            };

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        if let (Some(name), Some(codes)) = (name, codes) {
            Ok(Self {
                name,
                codes,
            })
        } else {
            Err(input.error("no name or code found."))
        }
    }
}

struct Target(Ident, Expr, Vec<Attribute>);

impl Target {
    fn ident(&self) -> &Ident {
        &self.0
    }

    fn val(&self) -> &Expr {
        &self.1
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
            let mut val = None;
            for attr in item.attrs.drain(..) {
                if attr.path.is_ident("event") {
                    val = Some(attr.parse_args::<Expr>()?);
                } else {
                    newattrs.push(attr);
                }
            }
            item.attrs = newattrs;
            if let Some(val) = val {
                let docs = item.attrs.iter().filter(|a| a.path.is_ident("doc")).cloned().collect::<Vec<_>>();
                result.push(Target(item.ident.clone(), val, docs));
            }
        }
    }

    Ok(result)
}

fn apply(attr: Args, item: &mut ItemMod) -> syn::Result<()> {
    let docs = item.attrs.iter().filter(|a| a.path.is_ident("doc")).cloned().collect::<Vec<_>>();

    let mut contents = if let Some((_, contents)) = &mut item.content {
        contents
    } else {
        return Err(syn::Error::new_spanned(item, "no mod body found."));
    };

    let targets = collect_targets(&mut contents)?;
    let events = targets.iter().map(Target::ident).collect::<Vec<_>>();
    let vals = targets.iter().map(Target::val).collect::<Vec<_>>();
    let tdocs = targets.iter().map(Target::docs).collect::<Vec<_>>();

    let name = &attr.name;
    let codes = &attr.codes;

    for event in &events {
        contents.push(parse_quote! {
            impl ::std::convert::From<#event> for #name {
                fn from(v: #event) -> Self {
                    Self::#event(v)
                }
            }
        });

        contents.push(parse_quote! {
            impl #event {
                pub const CODE: #codes = #codes::#event;
            }
        });
    }

    contents.push(parse_quote! {
        #( #docs )*
        #[derive(Debug, Clone)]
        pub enum #name {
            #( #( #tdocs )* #events(#events), )*
            Unknown(u16, Box<[u8]>),
        }
    });

    contents.push(parse_quote! {
        impl #name {
            #[doc(hidden)]
            pub fn unpack_inner<R>(code: #codes, read: &mut R) -> ::btmgmt_packet_helper::pack::Result<#name> where R: ::std::io::Read {
                Ok(match code {
                    #( #events::CODE => #name::#events(#events::unpack(read)?), )*
                })
            }
        }
    });

    contents.push(parse_quote! {
        /// Command Code
        #[derive(Debug, Clone, PartialEq, Eq, Hash, ::btmgmt_packet_helper::pack::Pack, ::btmgmt_packet_helper::pack::Unpack)]
        #[pack(u16)]
        pub enum #codes {
            #( #( #tdocs )* #events = #vals, )*
        }
    });

    Ok(())
}

pub fn events(attr: TokenStream, input: TokenStream) -> TokenStream {
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
