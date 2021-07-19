use proc_macro2::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Fields, GenericArgument, Type, parse_quote, Token, Ident};
use syn::visit::{self, Visit};
use syn::parse::{Parse, ParseStream};

fn assert(item: &DeriveInput) -> syn::Result<()> {
    match &item.data {
        Data::Struct(DataStruct {  fields: Fields::Unnamed(f), ..}) if f.unnamed.len() == 1 => {}
        _ => return Err(syn::Error::new_spanned(item, "expect newtype. named fields, enum or union not supported.")),
    }
    Ok(())
}

struct Conf {
    iter_mut: bool,
    item: Type,
    into_iter: Type,
}

impl Parse for Conf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut iter_mut = true;
        let mut item = None;
        let mut into_iter = None;

        while input.peek(Ident) {
            let name = input.parse::<Ident>()?;
            match name.to_string().as_str() {
                "item" => {
                    input.parse::<Token![=]>()?;
                    item = Some(input.parse::<Type>()?);
                }
                "into_iter" => {
                    input.parse::<Token![=]>()?;
                    into_iter = Some(input.parse::<Type>()?);
                }
                "no_iter_mut" => {
                    iter_mut = false;
                }
                unknown => return Err(input.error(format!("unknown attribute {}", unknown))),
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        if let (Some(item), Some(into_iter)) = (item, into_iter) {
            Ok(Conf {
                iter_mut,
                item,
                into_iter,
            })
        } else {
            Err(input.error("item or into_iter not found."))
        }
    }
}

struct FindFirstGenericParam<'b>(&'b mut Option<Type>);

impl<'ast, 'b> Visit<'ast> for FindFirstGenericParam<'b> {
    fn visit_generic_argument(&mut self, i: &'ast GenericArgument) {
        if let GenericArgument::Type(ty) = i {
            if self.0.is_none() {
                *self.0 = Some(ty.clone())
            }
        }
        visit::visit_generic_argument(self, i);
    }
}

fn detect_conf(item: &DeriveInput) -> syn::Result<Conf> {
    for attr in &item.attrs {
        if attr.path.is_ident("iter_newtype") {
            let conf = attr.parse_args::<Conf>()?;
            return Ok(conf);
        }
    }

    match &item.data {
        Data::Struct(DataStruct {  fields: Fields::Unnamed(f), ..}) if f.unnamed.len() == 1 => {
            let f = f.unnamed.first().unwrap();
            let mut ty = None;
            FindFirstGenericParam(&mut ty).visit_field(f);
            if let Some(ty) = ty {
                return Ok(Conf {
                    iter_mut: true,
                    into_iter: parse_quote! { ::std::vec::IntoIter<#ty> },
                    item: ty,
                });
            }
            todo!()
        }
        _ => unreachable!(),
    }
}

fn derive(input: TokenStream) -> syn::Result<TokenStream> {
    let item = syn::parse2::<DeriveInput>(input)?;
    assert(&item)?;
    let conf = detect_conf(&item)?;

    let ident = &item.ident;
    let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();
    let item = &conf.item;
    let into_iter = &conf.into_iter;

    let iter_mut = if conf.iter_mut {
        parse_quote! {
            pub fn iter_mut(&mut self) -> impl std::iter::Iterator<Item =&mut #item> {
                self.0.iter_mut()
            }
        }
    } else {
        TokenStream::new()
    };

    Ok(parse_quote! {
        impl #impl_generics ::std::iter::IntoIterator for #ident #type_generics #where_clause {
            type Item = #item;
            type IntoIter = #into_iter;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl #impl_generics ::std::iter::FromIterator<#item> for #ident #type_generics #where_clause {
            fn from_iter<T2F99A5F6AE614587BADEEAAB29145B70>(iter: T2F99A5F6AE614587BADEEAAB29145B70) -> Self where T2F99A5F6AE614587BADEEAAB29145B70: ::std::iter::IntoIterator<Item = #item> {
                Self(::std::iter::FromIterator::from_iter(iter))
            }
        }

        impl #impl_generics ::std::iter::Extend<#item> for #ident #type_generics #where_clause {
            fn extend<T789979AD04B840B9BCA2350BD2215CBC>(&mut self, iter: T789979AD04B840B9BCA2350BD2215CBC) where T789979AD04B840B9BCA2350BD2215CBC: ::std::iter::IntoIterator<Item = #item> {
                self.0.extend(iter)
            }
        }

        impl #impl_generics #ident #type_generics #where_clause {
            pub fn iter(&self) -> impl std::iter::Iterator<Item = &#item> {
                self.0.iter()
            }

            #iter_mut
        }
    })
}

pub fn iter_newtype(input: TokenStream) -> TokenStream {
    match derive(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}
