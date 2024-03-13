use proc_macro::{Ident, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn cat_ident(input: TokenStream) -> TokenStream {
    let mut ident_name = String::new();
    for token in input.into_iter() {
        if let TokenTree::Ident(ident) = token {
            ident_name += &ident.to_string();
        }
    }
    TokenStream::from(TokenTree::Ident(Ident::new(&ident_name, Span::call_site())))
}
