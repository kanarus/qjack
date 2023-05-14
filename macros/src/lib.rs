use proc_macro::TokenStream;

mod model;
#[proc_macro]
pub fn model(input: TokenStream) -> TokenStream {
    model::model(input.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
