use proc_macro2::TokenStream;
use syn::{Expr, Ident, Item, ItemMod, Token, parse_quote};
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

struct Target(Ident, Expr);

impl Target {
    fn ident(&self) -> &Ident {
        &self.0
    }

    fn val(&self) -> &Expr {
        &self.1
    }
}

fn collect_targets(items: &mut Vec<Item>) -> syn::Result<Vec<Target>> {
    let mut result = vec![];

    for item in items {
        if let Item::Struct(item) = item {
            let mut newattrs = vec![];
            for attr in item.attrs.drain(..) {
                if attr.path.is_ident("event") {
                    let val = attr.parse_args::<Expr>()?;
                    result.push(Target(item.ident.clone(), val));
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

    let targets = collect_targets(&mut contents)?;
    let events = targets.iter().map(Target::ident).collect::<Vec<_>>();
    let vals = targets.iter().map(Target::val).collect::<Vec<_>>();

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
        #[derive(Debug)]
        pub enum #name {
            #( #events(#events), )*
            Unknown(u16, Box<[u8]>),
        }
    });

    contents.push(parse_quote! {
        impl #name {
            pub fn unpack_inner<R>(code: #codes, read: &mut R) -> ::btmgmt_packet_helper::pack::Result<#name> where R: ::std::io::Read {
                Ok(match code {
                    #( #events::CODE => #name::#events(#events::unpack(read)?), )*
                })
            }
        }
    });

    contents.push(parse_quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, ::btmgmt_packet_helper::pack::Pack, ::btmgmt_packet_helper::pack::Unpack)]
        #[pack(u16)]
        pub enum #codes {
            #( #events = #vals, )*
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
