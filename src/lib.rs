extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Enum(ref data) => {
            if data.variants.iter().len() > 0 {
                let mut last : String = format!("{:?}", data.variants.iter().take(1).collect::<Vec<_>>().get(0).unwrap().ident);
                for v in data.variants.iter().skip(1) {
                    let id = v.ident.to_string();

                    if last > id {
                        panic!("Variant is not in order. Variant '{}' should come before '{}'", id, last);
                    }

                    last = id;
                }
            }
        },
        _ => unimplemented!()

    }

    TokenStream::new()
}
