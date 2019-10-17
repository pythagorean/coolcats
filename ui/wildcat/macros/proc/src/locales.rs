use crate::proc_macro::TokenStream;
use proc_quote::quote;

pub fn impl_uses_locale_values(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl UsesLocaleValues for #name {
            fn request_locale_values(&mut self, using_locale_values: Vec<String>) {
                self.context.send(context::Request::GetLocaleValues(using_locale_values));
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

macro_rules! quote_component_locale_update {
    () => {
        quote! {
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
}

pub fn impl_component_locale_update() -> TokenStream {
    quote_component_locale_update!().into()
}

pub fn impl_locale_component(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let component_locale_update = quote_component_locale_update!();
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
                component.request_locale_values(using_locale_values());
                component
            }

            #component_locale_update
        }
    };
    gen.into()
}
