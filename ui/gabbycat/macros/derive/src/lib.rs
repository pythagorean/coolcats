extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(UsesLocaleValues)]
pub fn uses_locale_values_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_uses_locale_values(&ast)
}

#[proc_macro_derive(UsesStateValues)]
pub fn uses_state_values_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_uses_state_values(&ast)
}

fn impl_uses_locale_values(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl UsesLocaleValues for #name {
            fn request_locale_values(&mut self) {
                self.context.send(context::Request::GetLocaleValues(using_locale_values()));
            }

            fn get_locale_value(&self, message_id: &str) -> &String {
                use lazy_static::*;
                lazy_static! {
                    static ref EMPTY: String = String::new();
                }
                self.locale_values.get(message_id).unwrap_or(&EMPTY)
            }
        }
    };
    gen.into()
}

fn impl_uses_state_values(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl UsesStateValues for #name {
            fn request_state_values(&mut self) {
                self.context.send(context::Request::GetSubstate(using_state_values()));
            }
        }
    };
    gen.into()
}
