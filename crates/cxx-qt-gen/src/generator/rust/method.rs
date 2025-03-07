// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::rust::get_params_tokens;
use crate::{
    generator::{naming::qobject::QObjectNames, rust::fragment::GeneratedRustFragment},
    parser::method::ParsedMethod,
};
use quote::quote;
use syn::{parse_quote_spanned, spanned::Spanned, Result};

pub fn generate_rust_methods(
    invokables: &[&ParsedMethod],
    qobject_names: &QObjectNames,
) -> Result<GeneratedRustFragment> {
    let cpp_class_name_rust = &qobject_names.name.rust_unqualified();

    let generated = invokables
        .iter()
        .map(|invokable| {
            // TODO: once we aren't using qobject::T in the extern "RustQt"
            // we can just pass through the original ExternFn block and add the attribute?
            let invokable_ident_cpp = invokable.name.cxx_unqualified();
            let invokable_ident_rust = invokable.name.rust_unqualified();

            let parameter_signatures = get_params_tokens(
                invokable.mutable,
                &invokable.parameters,
                cpp_class_name_rust,
            );

            let return_type = &invokable.method.sig.output;

            let cfgs = &invokable.cfgs;
            let cxx_namespace = qobject_names.namespace_tokens();

            let (block_type, block_safety) = if invokable.is_pure {
                ("C++", Some(quote! { unsafe }))
            } else {
                ("Rust", None)
            };
            // When generating extern Rust forward the block unsafe to fn
            // This allows for then defining pointer args when the whole block
            // is unsafe, as CXX does not allow for unsafe Rust
            let unsafe_call = if invokable.safe {
                if block_safety.is_none() && invokable.unsafe_block {
                    Some(quote! { unsafe })
                } else {
                    None
                }
            } else {
                Some(quote! { unsafe })
            };

            GeneratedRustFragment::from_cxx_item(parse_quote_spanned! {
                invokable.method.span() =>
                // Note: extern "Rust" block does not need to be unsafe
                #block_safety extern #block_type {
                    // Note that we are exposing a Rust method on the C++ type to C++
                    //
                    // CXX ends up generating the source, then we generate the matching header.
                    #[cxx_name = #invokable_ident_cpp]
                    // Needed for QObjects to have a namespace on their type or extern block
                    //
                    // A Namespace from cxx_qt::bridge would be automatically applied to all children
                    // but to apply it to only certain types, it is needed here too
                    #cxx_namespace
                    #(#cfgs)*
                    #[doc(hidden)]
                    #unsafe_call fn #invokable_ident_rust(#parameter_signatures) #return_type;
                }
            })
        })
        .collect::<Vec<_>>();

    Ok(GeneratedRustFragment::flatten(generated))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;
    use crate::tests::assert_tokens_eq;
    use syn::{parse_quote, ForeignItemFn};

    #[test]
    fn test_generate_rust_invokables() {
        let method1: ForeignItemFn = parse_quote! {
            #[cxx_name = "voidInvokable"]
            fn void_invokable(self: &MyObject);
        };
        let method2: ForeignItemFn = parse_quote! {
            #[cxx_name = "trivialInvokable"]
            fn trivial_invokable(self: &MyObject, param: i32) -> i32;
        };
        let method3: ForeignItemFn = parse_quote! {
            #[cxx_name = "opaqueInvokable"]
            fn opaque_invokable(self: Pin<&mut MyObject>, param: &QColor) -> UniquePtr<QColor>;
        };
        let method4: ForeignItemFn = parse_quote! {
            #[cxx_name = "unsafeInvokable"]
            unsafe fn unsafe_invokable(self: &MyObject, param: *mut T) -> *mut T;
        };
        let invokables = vec![
            ParsedMethod::mock_qinvokable(&method1),
            ParsedMethod::mock_qinvokable(&method2),
            ParsedMethod::mock_qinvokable(&method3).make_mutable(),
            ParsedMethod::mock_qinvokable(&method4).make_unsafe(),
        ];
        let qobject_names = create_qobjectname();

        let generated =
            generate_rust_methods(&invokables.iter().collect::<Vec<_>>(), &qobject_names).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 4);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 0);

        // void_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                extern "Rust" {
                    #[cxx_name = "voidInvokable"]
                    #[doc(hidden)]
                    fn void_invokable(self: &MyObject);
                }
            },
        );

        // trivial_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                extern "Rust" {
                    #[cxx_name = "trivialInvokable"]
                    #[doc(hidden)]
                    fn trivial_invokable(self: &MyObject, param: i32) -> i32;
                }
            },
        );

        // opaque_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[2],
            quote! {
                extern "Rust" {
                    #[cxx_name = "opaqueInvokable"]
                    #[doc(hidden)]
                    fn opaque_invokable(self: Pin<&mut MyObject>, param: &QColor) -> UniquePtr<QColor>;
                }
            },
        );

        // unsafe_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[3],
            quote! {
                extern "Rust" {
                    #[cxx_name = "unsafeInvokable"]
                    #[doc(hidden)]
                    unsafe fn unsafe_invokable(self:&MyObject, param: *mut T) -> *mut T;
                }
            },
        );
    }
}
