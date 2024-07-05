use convert_case::{Case, Casing};
use deluxe::HasAttributes;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{spanned::Spanned, ImplItem, ItemImpl};

pub fn impl_derive(_args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let mut input = syn::parse2::<ItemImpl>(input)?;

    let flags_impl = gen_flags_impl(&input);

    let mut flags = Vec::new();

    for item in input.items.iter() {
        let is_flag = item
            .attrs()
            .iter()
            .any(|attr| attr.path().is_ident("flag_mask"));
        if is_flag {
            process_flag(item, &mut flags)?;
        }
    }

    for flag_fn in flags {
        input.items.push(ImplItem::Verbatim(flag_fn));
    }

    Ok(quote! {
        #input

        #flags_impl
    })
}

fn gen_flags_impl(input: &ItemImpl) -> TokenStream {
    let struct_name = &input.self_ty;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics crate::BitFlagRegister for #struct_name #ty_generics #where_clause {
            fn get_flag(&self, flag: u8) -> bool {
                self.0 & flag == flag
            }

            fn set_flag(&mut self, flag: u8, value: bool) {
                if value {
                    self.0 |= flag;
                } else {
                    self.0 &= !flag;
                }
            }
        }
    }
}

fn process_flag(item: &ImplItem, flags: &mut Vec<TokenStream>) -> syn::Result<()> {
    let ImplItem::Const(item_const) = item else {
        return Err(syn::Error::new_spanned(
            item,
            "Only constants are allowed in a `#[bit_flag]` impl block",
        ));
    };

    let doc_comment = item_const
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .collect::<Vec<_>>();
    let const_name = &item_const.ident;
    let flag_name = const_name.to_string().to_case(Case::Snake);
    let getter_name = format_ident!("get_{flag_name}");
    let setter_name = format_ident!("set_{flag_name}");

    flags.push(quote_spanned! {item_const.span()=>
        #(#doc_comment)*
        pub fn #getter_name(&self) -> bool {
            self.get_flag(Self::#const_name)
        }
    });

    flags.push(quote_spanned! {item_const.span()=>
        #(#doc_comment)*
        pub fn #setter_name(&mut self, value: bool) {
            self.set_flag(Self::#const_name, value);
        }
    });

    Ok(())
}
