use proc_macro::{Literal, TokenStream, TokenTree};

#[proc_macro]
pub fn random(_input: TokenStream) -> TokenStream {
    use std::hash::{BuildHasher, Hasher};
    let val = std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish();
    let mut tokens = TokenStream::new();
    tokens.extend(Some(TokenTree::Literal(Literal::u64_unsuffixed(val))));
    tokens
}
