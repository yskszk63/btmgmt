use proc_macro::TokenStream;

mod commands;
mod events;
mod iter_newtype;
mod newtype;
mod pack;
mod unpack;

#[proc_macro_derive(Pack, attributes(pack))]
pub fn pack(input: TokenStream) -> TokenStream {
    pack::pack(input.into()).into()
}

#[proc_macro_derive(Unpack, attributes(pack))]
pub fn unpack(input: TokenStream) -> TokenStream {
    unpack::unpack(input.into()).into()
}

#[proc_macro_attribute]
pub fn commands(attr: TokenStream, input: TokenStream) -> TokenStream {
    commands::commands(attr.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn events(attr: TokenStream, input: TokenStream) -> TokenStream {
    events::events(attr.into(), input.into()).into()
}

#[proc_macro_derive(IterNewtype, attributes(iter_newtype))]
pub fn iter_newtype(input: TokenStream) -> TokenStream {
    iter_newtype::iter_newtype(input.into()).into()
}

#[proc_macro_derive(Newtype)]
pub fn newtype(input: TokenStream) -> TokenStream {
    newtype::newtype(input.into()).into()
}
