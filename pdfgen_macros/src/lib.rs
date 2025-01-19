use proc_macro::TokenStream;

mod name;

#[proc_macro]
pub fn const_names(token_stream: TokenStream) -> TokenStream {
    name::const_names(token_stream)
}
