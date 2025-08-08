use proc_macro::TokenStream;
use syn::{LitInt, parse_macro_input};

use crate::implementation::generate_constants;

mod implementation;

#[proc_macro]
pub fn constants(input: TokenStream) -> TokenStream {
    // Generate the constants module with the specified number of constants
    let num_consts = parse_macro_input!(input as LitInt)
        .base10_parse::<usize>()
        .unwrap();

    generate_constants(num_consts).into()
}
