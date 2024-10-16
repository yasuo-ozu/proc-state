use proc_macro::TokenStream;
use template_quote::quote;

use proc_state_testing_a::routing;
use proc_state_testing_macro::count;

#[proc_macro]
pub fn count2(_input: TokenStream) -> TokenStream {
    let n = count!();
    let m = routing();
    quote! {
        #{n + m}
    }
    .into()
}
