// For now variadic functions use macro_rules and boilerplate uses derive

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(UsesLocaleValues)]
pub fn uses_locale_values_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_uses_locale_values(&ast)
}

#[proc_macro_derive(LocaleComponent)]
pub fn locale_component_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_locale_component(&ast)
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

fn impl_locale_component(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        pub enum Msg {
            Context(context::Response),
        }

        impl Component for #name {
            type Message = Msg;
            type Properties = ();

            fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
                let mut component = Self {
                    context: context::Worker::bridge(link.send_back(Msg::Context)),
                    locale_values: HashMap::new(),
                };
                component.request_locale_values();
                component
            }

            fn update(&mut self, msg: Self::Message) -> ShouldRender {
                match msg {
                    Msg::Context(response) => match response {
                        context::Response::LocaleValues(locale_values) => {
                            self.locale_values = locale_values;
                            true
                        }
                        context::Response::Substate(_) => false,
                    },
                }
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
