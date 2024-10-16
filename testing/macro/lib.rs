use proc_macro::{Literal, TokenStream, TokenTree};
use proc_macro2::TokenStream as TokenStream2;
use proc_state::Global;
use std::sync::Mutex;
use template_quote::quote;

const COUNTER: Global<Mutex<usize>> = proc_state::new!();

#[proc_macro]
pub fn count(input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();
    let mx = COUNTER.or_insert(Mutex::new(0));
    let mut r = mx.lock().unwrap();
    *r += 1;
    quote! {
        {
            {
                #input
            }
            #{*r}
        }
    }
    .into()
}
