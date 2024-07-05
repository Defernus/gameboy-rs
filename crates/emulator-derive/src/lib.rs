use proc_macro_error::proc_macro_error;

mod bit_flag;
mod control_register;

#[proc_macro_error]
#[proc_macro_derive(ControlRegister, attributes(register))]
pub fn derive_control_register(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    control_register::impl_derive(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn bit_flag(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    bit_flag::impl_derive(args.into(), input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// This attribute macro is used to mark a constant as a flag mask.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn flag_mask(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}
