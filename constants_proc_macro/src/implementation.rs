use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitInt};

pub fn generate_constants(num_consts: usize) -> TokenStream {
    let num_consts_lit = LitInt::new(&num_consts.to_string(), Span::call_site());

    let constants = (0..num_consts)
        .map(|i| {
            let ident: Ident = Ident::new(format!("CONST_{i}").as_str(), Span::call_site());
            let name = format!("CONST_{i}");
            let value = LitInt::new(&i.to_string(), Span::call_site());
            quote! {
                pub const #ident: Constant = Constant { name: #name, value: #value };
            }
        })
        .collect::<Vec<_>>();

    let idents = (0..num_consts)
        .map(|i| Ident::new(format!("CONST_{i}").as_str(), Span::call_site()))
        .collect::<Vec<_>>();

    quote! {
        mod consts {
            pub struct Constant {
                pub name: &'static str,
                pub value: i32,
            }

            #(#constants)*
        }

        const all_consts: [consts::Constant; #num_consts_lit] = {
            use consts::*;
            [#(#idents),*]
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_constants() {
        let num_consts = 5;
        let generated = generate_constants(num_consts);
        let expected = quote! {
            mod consts {
                pub struct Constant {
                    pub name: &'static str,
                    pub value: i32,
                }

                pub const CONST_0: Constant = Constant { name: "CONST_0", value: 0 };
                pub const CONST_1: Constant = Constant { name: "CONST_1", value: 1 };
                pub const CONST_2: Constant = Constant { name: "CONST_2", value: 2 };
                pub const CONST_3: Constant = Constant { name: "CONST_3", value: 3 };
                pub const CONST_4: Constant = Constant { name: "CONST_4", value: 4 };
            }

            const all_consts: [consts::Constant; 5] = {
                use consts::*;
                [CONST_0, CONST_1, CONST_2, CONST_3, CONST_4]
            };
        };
        assert_eq!(generated.to_string(), expected.to_string());
    }
}
