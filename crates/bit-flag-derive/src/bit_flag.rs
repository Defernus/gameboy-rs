use convert_case::{Case, Casing};
use deluxe::HasAttributes;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{Attribute, Expr, ExprLit, Ident, ImplItem, ItemImpl, Meta, Type};

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

        let value_attr = item
            .attrs()
            .iter()
            .find(|attr| attr.path().is_ident("value_mask"));
        if let Some(value_attr) = value_attr {
            process_value(item, &mut flags, value_attr)?;
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
        impl #impl_generics bit_flag::BitFlagRegister for #struct_name #ty_generics #where_clause {
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

fn prepare_const_mask(item: &ImplItem) -> syn::Result<ConstData> {
    let ImplItem::Const(item_const) = item else {
        return Err(syn::Error::new_spanned(
            item,
            "Only constants are allowed in a `#[flag_mask]` impl block",
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
    let const_value = &item_const.expr;

    Ok(ConstData {
        doc_comment,
        const_name,
        getter_name,
        setter_name,
        const_value,
    })
}

struct ConstData<'a> {
    doc_comment: Vec<&'a Attribute>,
    const_name: &'a Ident,
    getter_name: Ident,
    setter_name: Ident,
    const_value: &'a Expr,
}

fn process_flag(item: &ImplItem, flags: &mut Vec<TokenStream>) -> syn::Result<()> {
    let ConstData {
        doc_comment,
        const_name,
        getter_name,
        setter_name,
        const_value: _,
    } = prepare_const_mask(item)?;

    flags.push(quote_spanned! {const_name.span()=>
        #(#doc_comment)*
        pub fn #getter_name(&self) -> bool {
            bit_flag::BitFlagRegister::get_flag(self, Self::#const_name)
        }
    });

    flags.push(quote_spanned! {const_name.span()=>
        #(#doc_comment)*
        pub fn #setter_name(&mut self, value: bool) {
            bit_flag::BitFlagRegister::set_flag(self, Self::#const_name, value);
        }
    });

    Ok(())
}

fn process_value(
    item: &ImplItem,
    flags: &mut Vec<TokenStream>,
    value_attr: &Attribute,
) -> syn::Result<()> {
    let mapped_value_ty = parse_value_attr(value_attr)?;

    let ConstData {
        doc_comment,
        const_name,
        getter_name,
        setter_name,
        const_value,
    } = prepare_const_mask(item)?;

    let mask = expr_to_mask(const_value)?;

    let shifts = u8_to_shifts(mask);
    let bits_amount = shifts.len();
    if bits_amount > 7 {
        return Err(syn::Error::new_spanned(
            const_value,
            "Value mask can't have more than 7 bits, just use u8",
        ));
    }

    if bits_amount == 0 {
        return Err(syn::Error::new_spanned(
            const_value,
            "Value mask should have at least one bit",
        ));
    }

    let value_ty = format_ident!("U{}", bits_amount);
    let value_ty = quote_spanned! {const_name.span()=>
        bit_flag::#value_ty
    };

    let mut getter_body = Vec::with_capacity(shifts.len());
    let mut setter_body = Vec::with_capacity(shifts.len());

    // TODO optimize bits operations for masks with consecutive bits (like 0b0000_1111 of 0b0011_1100)
    for (
        i,
        BitShiftData {
            mask,
            shift: r_shift,
        },
    ) in shifts.into_iter().enumerate()
    {
        getter_body.push(quote_spanned! {const_name.span()=>
            ((self.0 & #mask) >> #r_shift)
        });

        setter_body.push(quote_spanned! {const_name.span()=>
            (value & (1 << #i)) << #r_shift
        })
    }

    if let Some(mapped_value_ty) = mapped_value_ty {
        flags.push(quote_spanned! {const_name.span()=>
            #(#doc_comment)*
            pub fn #getter_name(&self) -> #mapped_value_ty {
                let value: #value_ty = (#(#getter_body)|*).into();
                From::from(value)
            }
        });

        flags.push(quote_spanned! {const_name.span()=>
            #(#doc_comment)*
            pub fn #setter_name(&mut self, value: #mapped_value_ty) {
                let prev_value: u8 = (*self).into();
                let value: #value_ty = value.into();
                let value: u8 = value.into();
                let value = #(#setter_body)|*;

                *self = ((prev_value & !#mask) | value).into();
            }
        });
    } else {
        flags.push(quote_spanned! {const_name.span()=>
            #(#doc_comment)*
            pub fn #getter_name(&self) -> #value_ty {
                (#(#getter_body)|*).into()
            }
        });

        flags.push(quote_spanned! {const_name.span()=>
            #(#doc_comment)*
            pub fn #setter_name(&mut self, value: #value_ty) {
                let prev_value: u8 = (*self).into();
                let value: u8 = value.into();
                let value = #(#setter_body)|*;

                *self = ((prev_value & !#mask) | value).into();
            }
        });
    }

    Ok(())
}

/// Parses the `#[value_mask(ResultType)]` attribute.
fn parse_value_attr(value_attr: &Attribute) -> syn::Result<Option<Type>> {
    let list = match &value_attr.meta {
        Meta::List(list) => list,
        Meta::Path(_) => return Ok(None),
        _ => {
            return Err(syn::Error::new_spanned(
                value_attr,
                "Expected `#[value_mask(ResultType)]` format",
            ))
        }
    };

    Ok(Some(list.parse_args()?))
}

const EXPECTED_VALUE_FORMAT_ERR: &str = "Expected a value in the format `0bxxxx_xxxx`";

fn expr_to_mask(const_value: &Expr) -> syn::Result<u8> {
    let Expr::Lit(ExprLit { lit, .. }) = const_value else {
        return Err(syn::Error::new_spanned(
            const_value,
            EXPECTED_VALUE_FORMAT_ERR,
        ));
    };

    let syn::Lit::Int(lit_int) = lit else {
        return Err(syn::Error::new_spanned(
            const_value,
            EXPECTED_VALUE_FORMAT_ERR,
        ));
    };

    lit_int.base10_parse::<u8>()
}

fn u8_to_shifts(mask: u8) -> Vec<BitShiftData> {
    let mut bits_count: u8 = 0;
    (0..8u8)
        .filter_map(|bit_pos| {
            if mask & 1 << bit_pos == 0 {
                return None;
            }
            let shift = bit_pos - bits_count;

            bits_count += 1;
            Some(BitShiftData {
                shift,
                mask: 1 << bit_pos,
            })
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq)]
struct BitShiftData {
    mask: u8,
    shift: u8,
}

#[test]
fn test_u8_to_shifts() {
    let mask = 0b0010_1100;
    let shifts = u8_to_shifts(mask);
    assert_eq!(
        shifts,
        [
            BitShiftData {
                mask: 0b100,
                shift: 2
            },
            BitShiftData {
                mask: 0b1000,
                shift: 2
            },
            BitShiftData {
                mask: 0b10_0000,
                shift: 3
            }
        ]
    );
}
