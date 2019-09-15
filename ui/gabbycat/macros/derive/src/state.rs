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
