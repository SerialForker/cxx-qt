// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::{fragment::CppFragment, GeneratedCppBlocks};
use crate::writer::cpp::namespace_pair;
use indoc::formatdoc;

/// Extract the header from a given CppFragment
fn pair_as_header(pair: &CppFragment) -> Option<String> {
    match pair {
        CppFragment::Pair { header, source: _ } => Some(header.clone()),
        CppFragment::Header(header) => Some(header.clone()),
        CppFragment::Source(_) => None,
    }
}

/// With a given block name, join the given items and add them under the block
fn create_block(block: &str, items: &[String]) -> String {
    if items.is_empty() {
        "".to_owned()
    } else {
        formatdoc! {r#"
        {block}:
          {items}
        "#,
            block = block,
            items = items
                .iter()
                // Remove any empty entries from the blocks
                .filter(|item| !item.is_empty())
                .cloned()
                .collect::<Vec<String>>()
                .join("\n  "),
        }
    }
}

/// For a given GeneratedCppBlocks write the forward declare
fn forward_declare(generated: &GeneratedCppBlocks) -> Vec<String> {
    let (namespace_start, namespace_end) = namespace_pair(generated);

    generated
        .qobjects
        .iter()
        .map(|qobject| {
            formatdoc! { r#"
                {namespace_start}
                class {ident};
                {forward_declares}
                {namespace_end}
            "#,
            ident = &qobject.ident,
            forward_declares = qobject.blocks.forward_declares.join("\n"),
            namespace_start = namespace_start,
            namespace_end = namespace_end,
            }
        })
        .collect::<Vec<String>>()
}

/// For a given GeneratedCppBlocks write the classes
fn qobjects_header(generated: &GeneratedCppBlocks) -> Vec<String> {
    let (namespace_start, namespace_end) = namespace_pair(generated);

    generated.qobjects.iter().map(|qobject| {
        formatdoc! { r#"
            {namespace_start}
            class {ident} : public {base_class}
            {{
              Q_OBJECT
              {metaobjects}

            public:
              explicit {ident}(QObject* parent = nullptr);
              ~{ident}();
              {rust_ident} const& unsafeRust() const;
              {rust_ident}& unsafeRustMut();

            {methods}
            private:
              {members}
            }};

            static_assert(::std::is_base_of<QObject, {ident}>::value, "{ident} must inherit from QObject");
            {namespace_end}

            Q_DECLARE_METATYPE({metatype}*)
        "#,
        ident = qobject.ident,
        namespace_start = namespace_start,
        namespace_end = namespace_end,
        rust_ident = qobject.rust_ident,
        base_class = qobject.base_class,
        metaobjects = qobject.blocks.metaobjects.join("\n  "),
        methods = create_block("public", &qobject.blocks.methods.iter().filter_map(pair_as_header).collect::<Vec<String>>()),
        members = {
            let mut members = vec![
                format!("::rust::Box<{rust_ident}> m_rustObj;", rust_ident = qobject.rust_ident),
            ];

            if qobject.locking {
                members.extend(vec![
                    "::std::shared_ptr<::std::recursive_mutex> m_rustObjMutex;".to_string(),
                ]);
            }

            members.extend(qobject.blocks.members.iter().filter_map(pair_as_header).collect::<Vec<String>>());
            members.join("\n  ")
        },
        metatype = if generated.namespace.is_empty() {
            qobject.ident.clone()
        } else {
            format!("{namespace}::{ident}", namespace = generated.namespace, ident = qobject.ident)
        },
        }
    }).collect::<Vec<String>>()
}

/// For a given GeneratedCppBlocks write this into a C++ header
pub fn write_cpp_header(generated: &GeneratedCppBlocks) -> String {
    // Headers included:
    // <memory> - unique_ptr to the Rust object.
    // <mutex> - used for mutex locking the rust object.
    formatdoc! {r#"
        #pragma once

        #include <memory>
        #include <mutex>

        namespace rust::cxxqtlib1 {{
        template<typename T>
        class CxxQtThread;
        }}

        {forward_declare}
        #include "cxx-qt-gen/{cxx_file_stem}.cxx.h"

        {qobjects}
    "#,
    cxx_file_stem = generated.cxx_file_stem,
    forward_declare = forward_declare(generated).join("\n"),
    qobjects = qobjects_header(generated).join("\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::writer::cpp::tests::{
        create_generated_cpp, create_generated_cpp_multi_qobjects,
        create_generated_cpp_no_namespace, expected_header, expected_header_multi_qobjects,
        expected_header_no_namespace,
    };
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_create_block() {
        let block = create_block("block", &["line1".to_string(), "line2".to_string()]);
        let expected = indoc! {"
        block:
          line1
          line2
        "};
        assert_str_eq!(block, expected);
    }

    #[test]
    fn test_create_block_with_empty() {
        let block = create_block(
            "block",
            &["line1".to_string(), "".to_string(), "line2".to_string()],
        );
        let expected = indoc! {"
        block:
          line1
          line2
        "};
        assert_str_eq!(block, expected);
    }

    #[test]
    fn test_write_cpp_header() {
        let generated = create_generated_cpp();
        let output = write_cpp_header(&generated);
        assert_str_eq!(output, expected_header());
    }

    #[test]
    fn test_write_cpp_header_multi_qobjects() {
        let generated = create_generated_cpp_multi_qobjects();
        let output = write_cpp_header(&generated);
        assert_str_eq!(output, expected_header_multi_qobjects());
    }

    #[test]
    fn test_write_cpp_header_no_namespace() {
        let generated = create_generated_cpp_no_namespace();
        let output = write_cpp_header(&generated);
        assert_str_eq!(output, expected_header_no_namespace());
    }
}
