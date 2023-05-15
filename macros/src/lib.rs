use proc_macro::TokenStream;

mod model;
#[proc_macro_attribute]
pub fn model(_: TokenStream, input: TokenStream) -> TokenStream {
    model::model(input.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
