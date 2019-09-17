extern crate proc_macro;
mod locales;
mod state;

use crate::proc_macro::TokenStream;

#[proc_macro_derive(UsesLocaleValues)]
pub fn uses_locale_values_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    locales::impl_uses_locale_values(&ast)
}

#[proc_macro]
pub fn component_locale_update(_: TokenStream) -> TokenStream {
    locales::impl_component_locale_update()
}

#[proc_macro_derive(LocaleComponent)]
pub fn locale_component_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    locales::impl_locale_component(&ast)
}

#[proc_macro_derive(UsesStateValues)]
pub fn uses_state_values_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    state::impl_uses_state_values(&ast)
}
