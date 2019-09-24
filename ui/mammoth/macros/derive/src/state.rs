use crate::proc_macro::TokenStream;
use quote::quote;

pub fn impl_uses_state_values(ast: &syn::DeriveInput) -> TokenStream {
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

macro_rules! quote_component_state_update {
    () => {
        quote! {
            fn update(&mut self, msg: Self::Message) -> ShouldRender {
                match msg {
                    Msg::Context(response) => match response {
                        context::Response::Substate(substate) => {
                            self.substate = substate;
                            true
                        }
                        context::Response::LocaleValues(_) => false,
                    },
                }
            }
        }
    };
}

pub fn impl_component_state_update() -> TokenStream {
    quote_component_state_update!().into()
}

pub fn impl_state_component(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let component_state_update = quote_component_state_update!();
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
                    substate: State::unset(),
                };
                component.request_state_values();
                component
            }

            #component_state_update
        }
    };
    gen.into()
}
