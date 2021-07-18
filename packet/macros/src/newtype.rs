use proc_macro2::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Fields, Type, parse_quote};

fn assert(item: &DeriveInput) -> syn::Result<()> {
    match &item.data {
        Data::Struct(DataStruct {  fields: Fields::Unnamed(f), ..}) if f.unnamed.len() == 1 => {}
        _ => return Err(syn::Error::new_spanned(item, "expect newtype. named fields, enum or union not supported.")),
    }
    Ok(())
}

struct Conf {
    item: Type,
}

fn detect_conf(item: &DeriveInput) -> syn::Result<Conf> {
    match &item.data {
        Data::Struct(DataStruct {  fields: Fields::Unnamed(f), ..}) if f.unnamed.len() == 1 => {
            let ty = f.unnamed.first().unwrap().ty.clone();
            Ok(Conf {
                item: ty,
            })
        }
        _ => unreachable!(),
    }
}

fn derive(input: TokenStream) -> syn::Result<TokenStream> {
    let item = syn::parse2::<DeriveInput>(input)?;
    assert(&item)?;
    let conf = detect_conf(&item)?;

    let ident = &item.ident;
    let item = &conf.item;

    Ok(parse_quote! {
        impl ::std::convert::From<#item> for #ident {
            fn from(v: #item) -> Self {
                Self(v)
            }
        }

        impl ::std::convert::AsRef<#item> for #ident {
            fn as_ref(&self) -> &#item {
                &self.0
            }
        }

        impl ::std::convert::AsMut<#item> for #ident {
            fn as_mut(&mut self) -> &mut #item {
                &mut self.0
            }
        }

        impl ::std::ops::Deref for #ident {
            type Target = #item;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for #ident {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    })
}

pub fn newtype(input: TokenStream) -> TokenStream {
    match derive(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}
