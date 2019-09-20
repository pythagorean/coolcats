use crate::proc_macro::TokenStream;
use quote::quote;

pub fn impl_props_component(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        pub enum Msg {}

        impl Component for #name {
            type Message = Msg;
            type Properties = Props;

            fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
                Self {
                    props,
                }
            }

            fn update(&mut self, _msg: Self::Message) -> ShouldRender {
                false
            }

            fn change(&mut self, props: Self::Properties) -> ShouldRender {
                self.props = props;
                true
            }
        }
    };
    gen.into()
}

pub fn impl_impl_component(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        pub enum Msg {}

        impl Component for #name {
            type Message = Msg;
            type Properties = ();

            fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
                Self
            }

            fn update(&mut self, _msg: Self::Message) -> ShouldRender {
                false
            }
        }
    };
    gen.into()
}
