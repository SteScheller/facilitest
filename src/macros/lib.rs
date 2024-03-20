use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use std::iter;

#[proc_macro]
pub fn build_ident(input: TokenStream) -> TokenStream {
    let mut ident_name = String::new();
    for token in input.into_iter() {
        match token {
            TokenTree::Ident(ident) => ident_name += &ident.to_string(),
            TokenTree::Group(group) => {
                if let Some(ident) = group.stream().into_iter().last() {
                    ident_name += &ident.to_string();
                }
            }
            _ => (),
        }
    }
    TokenStream::from(TokenTree::Ident(Ident::new(&ident_name, Span::call_site())))
}

#[proc_macro]
pub fn make_test_fn(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let mut ts = TokenStream::new();

    // unwrap macro arguments
    let func = tokens.next().unwrap();
    let _ = tokens.next().unwrap();
    let suffix = tokens.next().unwrap();
    let _ = tokens.next().unwrap();
    let args = tokens.next().unwrap();
    let _ = tokens.next().unwrap();
    let expected = tokens.next().unwrap();

    // start with "test" attribute
    let ts_attr = TokenStream::from_iter([
        TokenTree::from(Punct::new('#', Spacing::Alone)),
        TokenTree::from(Group::new(
            Delimiter::Bracket,
            TokenStream::from(TokenTree::from(Ident::new("test", Span::call_site()))),
        )),
    ]);
    ts.extend(ts_attr);

    // append "fn" keyword
    ts.extend(iter::once(TokenTree::from(Ident::new(
        "fn",
        Span::call_site(),
    ))));

    // append function name
    let ts_name = TokenStream::from_iter([
        TokenTree::from(Ident::new("test_", Span::call_site())),
        func.clone(),
        suffix,
    ]);
    let name = build_ident(ts_name);
    ts.extend(iter::once(name));

    // add empty parameter list
    ts.extend(iter::once(TokenTree::from(Group::new(
        Delimiter::Parenthesis,
        TokenStream::new(),
    ))));

    // add function body
    let mut ts_body = TokenStream::new();
    if let TokenTree::Group(g_args) = args {
        ts_body.extend([
            TokenTree::from(Ident::new("assert_eq", Span::call_site())),
            TokenTree::from(Punct::new('!', Spacing::Alone)),
            TokenTree::from(Group::new(
                Delimiter::Parenthesis,
                TokenStream::from_iter([
                    func,
                    TokenTree::from(Group::new(
                        Delimiter::Parenthesis,
                        g_args.stream()
                    )),
                    TokenTree::from(Punct::new(',', Spacing::Alone)),
                    expected,
                ]),
            )),
            TokenTree::from(Punct::new(';', Spacing::Alone)),
        ]);
    }

    ts.extend(iter::once(TokenTree::from(Group::new(
        Delimiter::Brace,
        ts_body,
    ))));

    ts
}
