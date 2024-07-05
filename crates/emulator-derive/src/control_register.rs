use deluxe::ParseAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr};

#[derive(Debug, Clone, ParseAttributes)]
#[deluxe(attributes(register))]
struct RegisterOptions {
    address: Expr,
}

pub fn impl_derive(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<DeriveInput>(input)?;
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let RegisterOptions { address } = deluxe::parse_attributes(&input)?;

    Ok(quote! {
        impl #impl_generics crate::ControlRegister for #struct_name #ty_generics #where_clause {
            const ADDRESS: u16 = #address;

            fn from_memory_mut(emulator: &mut Emulator) -> &mut Self {
                let data = emulator.get_mut(Self::ADDRESS);

                // Safety: `RegisterLCDC` has the same memory layout as `u8`
                unsafe { std::mem::transmute(data) }
            }
        }

        impl #impl_generics From<u8> for #struct_name #ty_generics #where_clause {
            fn from(value: u8) -> Self {
                Self(value)
            }
        }

        impl #impl_generics From<#struct_name> for u8 #ty_generics #where_clause {
            fn from(value: #struct_name) -> u8 {
                value.0
            }
        }
    })
}
