use proc_macro2::TokenStream;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Ident};
use quote::quote;

fn derive_unit(item: &DeriveInput, _: &DataStruct) -> syn::Result<TokenStream> {
    let ident = &item.ident;

    let code = quote! {
        impl ::btmgmt_packet_helper::pack::Unpack for #ident {
            fn unpack<R>(_: &mut R) -> ::btmgmt_packet_helper::pack::Result<Self> where R: ::std::io::Read {
                Ok(Self)
            }
        }
    };
    Ok(code)
}

fn derive_tuple(item: &DeriveInput, data: &DataStruct) -> syn::Result<TokenStream> {
    let ident = &item.ident;
    let fields = (0..data.fields.len()).map(|_| TokenStream::new()).collect::<Vec<_>>();
    let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

    let code = quote! {
        impl #impl_generics ::btmgmt_packet_helper::pack::Unpack for #ident #type_generics #where_clause {
            fn unpack<R>(read: &mut R) -> ::btmgmt_packet_helper::pack::Result<Self> where R: ::std::io::Read {
                Ok(Self(
                    #( #fields ::btmgmt_packet_helper::pack::Unpack::unpack(read)?, )*
                ))
            }
        }
    };
    Ok(code)
}

fn derive_standard(item: &DeriveInput, data: &DataStruct) -> syn::Result<TokenStream> {
    let ident = &item.ident;
    let fields = data.fields.iter().map(|f| f.ident.as_ref().unwrap()).collect::<Vec<_>>();
    let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

    let code = quote! {
        impl #impl_generics ::btmgmt_packet_helper::pack::Unpack for #ident #type_generics #where_clause {
            fn unpack<R>(read: &mut R) -> ::btmgmt_packet_helper::pack::Result<Self> where R: ::std::io::Read {
                Ok(Self {
                    #( #fields: ::btmgmt_packet_helper::pack::Unpack::unpack(read)?, )*
                })
            }
        }
    };
    Ok(code)
}

fn derive_enum(item: &DeriveInput, data: &DataEnum) -> syn::Result<TokenStream> {
    let mut ty = None;
    for attr in &item.attrs {
        if attr.path.is_ident("pack") {
            ty = Some(attr.parse_args::<Ident>()?);
        }
    }
    let ty = if let Some(ty) = ty {
        ty
    } else {
        return Err(syn::Error::new_spanned(item, "no `pack` specified."));
    };

    let ident = &item.ident;
    let fields = data.variants.iter().map(|v| {
        if let Fields::Unit = v.fields {
            Ok(&v.ident)
        } else {
            Err(syn::Error::new_spanned(v, "supports unit variant only."))
        }
    }).collect::<syn::Result<Vec<_>>>()?;

    let code = quote! {
        impl ::btmgmt_packet_helper::pack::Unpack for #ident {
            fn unpack<R>(read: &mut R) -> ::btmgmt_packet_helper::pack::Result<Self> where R: ::std::io::Read {
                #![allow(non_upper_case_globals)]
                #( const #fields: #ty = #ident::#fields as #ty; )*
                Ok(match #ty::unpack(read)? {
                    #( #fields => Self::#fields, )*
                    unknown => return Err(::btmgmt_packet_helper::pack::Error::UnexpectedValue(format!("{}", unknown))),
                })
            }
        }
    };
    Ok(code)
}

fn derive(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<DeriveInput>(input)?;
    match &input.data {
        Data::Struct(data @ DataStruct { fields: Fields::Unit ,.. } ) => derive_unit(&input, data),
        Data::Struct(data @ DataStruct { fields: Fields::Unnamed(..) ,.. } ) => derive_tuple(&input, data),
        Data::Struct(data @ DataStruct { fields: Fields::Named(..) ,.. } ) => derive_standard(&input, data),
        Data::Enum(data) => derive_enum(&input, data),
        _ => todo!()
    }
}

pub fn unpack(input: TokenStream) -> TokenStream {
    match derive(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}
