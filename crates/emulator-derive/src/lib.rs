use proc_macro_error::proc_macro_error;

mod control_register;

#[proc_macro_error]
#[proc_macro_derive(ControlRegister, attributes(register))]
pub fn derive_control_register(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    control_register::impl_derive(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
