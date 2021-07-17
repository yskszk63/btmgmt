use proc_macro::TokenStream;

mod pack;
mod unpack;
mod commands;
mod events;

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
