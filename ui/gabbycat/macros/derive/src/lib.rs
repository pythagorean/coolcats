extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(UsesLocaleValues)]
pub fn uses_locale_values_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_uses_locale_values(&ast)
}

fn impl_uses_locale_values(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl UsesLocaleValues for #name {
            fn request_locale_values(&mut self) {
                self.context.send(context::Request::GetLocaleValues(using_locale_values()));
            }

            fn get_locale_value(&self, message_id: &str) -> &String {
                lazy_static! {
                    static ref EMPTY: String = String::new();
                }
                self.locale_values.get(message_id).unwrap_or(&EMPTY)
            }
        }
    };
    gen.into()
}
