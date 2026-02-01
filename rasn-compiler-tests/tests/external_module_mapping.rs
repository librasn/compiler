//! Tests for external module mapping feature.
//!
//! This feature allows mapping ASN.1 module names to external Rust crate paths,
//! so that IMPORTS from those modules generate `use crate_path::Type;` instead
//! of `use super::module_name::Type;`.

use std::collections::HashMap;

use rasn_compiler::prelude::*;

/// Test that external module mapping generates correct import statements.
#[test]
fn external_module_mapping_generates_correct_import() {
    let mut config = RasnConfig::default();

    // Map PKIX1Explicit88 to rasn_pkix crate
    config.external_module_mappings.insert(
        "PKIX1Explicit88".to_string(),
        ExternalModuleMapping {
            rust_path: "rasn_pkix".to_string(),
            type_mappings: HashMap::new(),
        },
    );

    let asn1 = r#"
        TestModule DEFINITIONS AUTOMATIC TAGS ::= BEGIN
            IMPORTS Certificate FROM PKIX1Explicit88 { 1 2 3 };

            MyCert ::= Certificate
        END
    "#;

    let result = Compiler::<RasnBackend, _>::new_with_config(config)
        .add_asn_literal(asn1)
        .compile_to_string()
        .unwrap();

    // Verify the generated code uses external crate import
    assert!(
        result.generated.contains("use rasn_pkix::Certificate;"),
        "Expected external crate import, got:\n{}",
        result.generated
    );

    // Verify it does NOT use sibling module import
    assert!(
        !result.generated.contains("use super::pkix1_explicit88"),
        "Should not have sibling module import, got:\n{}",
        result.generated
    );
}

/// Test external module mapping with multiple types imported.
#[test]
fn external_module_mapping_multiple_types() {
    let mut config = RasnConfig::default();

    config.external_module_mappings.insert(
        "PKIX1Explicit88".to_string(),
        ExternalModuleMapping {
            rust_path: "rasn_pkix".to_string(),
            type_mappings: HashMap::new(),
        },
    );

    let asn1 = r#"
        TestModule DEFINITIONS AUTOMATIC TAGS ::= BEGIN
            IMPORTS Certificate, CertificateList, Time FROM PKIX1Explicit88 { 1 2 3 };

            MyCert ::= Certificate
            MyList ::= CertificateList
        END
    "#;

    let result = Compiler::<RasnBackend, _>::new_with_config(config)
        .add_asn_literal(asn1)
        .compile_to_string()
        .unwrap();

    // Verify all types are imported from external crate (order may vary after formatting)
    assert!(
        result.generated.contains("use rasn_pkix::")
            && result.generated.contains("Certificate")
            && result.generated.contains("CertificateList")
            && result.generated.contains("Time"),
        "Expected all types from external crate, got:\n{}",
        result.generated
    );

    // Verify no sibling imports for PKIX
    assert!(
        !result.generated.contains("super::pkix1_explicit88"),
        "Should not have sibling module import"
    );
}

/// Test external module mapping with explicit type name mapping.
#[test]
fn external_module_mapping_with_type_mapping() {
    let mut config = RasnConfig::default();

    let mut type_mappings = HashMap::new();
    // Map ASN.1 type name to different Rust type name
    type_mappings.insert("MyAsn1Type".to_string(), "RustTypeName".to_string());

    config.external_module_mappings.insert(
        "ExternalModule".to_string(),
        ExternalModuleMapping {
            rust_path: "my_crate::submodule".to_string(),
            type_mappings,
        },
    );

    let asn1 = r#"
        TestModule DEFINITIONS AUTOMATIC TAGS ::= BEGIN
            IMPORTS MyAsn1Type FROM ExternalModule { 1 2 3 };

            MyType ::= MyAsn1Type
        END
    "#;

    let result = Compiler::<RasnBackend, _>::new_with_config(config)
        .add_asn_literal(asn1)
        .compile_to_string()
        .unwrap();

    // Verify the mapped type name is used in the import
    assert!(
        result.generated.contains("use my_crate::submodule::RustTypeName;"),
        "Expected mapped type name in import, got:\n{}",
        result.generated
    );
}

/// Test that unmapped modules still use default sibling import.
#[test]
fn unmapped_module_uses_sibling_import() {
    let mut config = RasnConfig::default();

    // Only map one module
    config.external_module_mappings.insert(
        "MappedModule".to_string(),
        ExternalModuleMapping {
            rust_path: "mapped_crate".to_string(),
            type_mappings: HashMap::new(),
        },
    );

    // Note: multiple IMPORTS statements need to be combined in ASN.1
    let asn1 = r#"
        TestModule DEFINITIONS AUTOMATIC TAGS ::= BEGIN
            IMPORTS
                MappedType FROM MappedModule { 1 2 3 }
                UnmappedType FROM UnmappedModule { 1 2 3 };

            MyType1 ::= MappedType
            MyType2 ::= UnmappedType
        END
    "#;

    let result = Compiler::<RasnBackend, _>::new_with_config(config)
        .add_asn_literal(asn1)
        .compile_to_string()
        .unwrap();

    // Verify mapped module uses external import
    assert!(
        result.generated.contains("use mapped_crate::"),
        "Expected external crate import for mapped module, got:\n{}",
        result.generated
    );

    // Verify unmapped module uses sibling import
    assert!(
        result.generated.contains("use super::unmapped_module::"),
        "Expected sibling import for unmapped module, got:\n{}",
        result.generated
    );
}

/// Test external module mapping with lowercase constant imports.
#[test]
fn external_module_mapping_with_constant() {
    let mut config = RasnConfig::default();

    config.external_module_mappings.insert(
        "ConstModule".to_string(),
        ExternalModuleMapping {
            rust_path: "const_crate".to_string(),
            type_mappings: HashMap::new(),
        },
    );

    let asn1 = r#"
        TestModule DEFINITIONS AUTOMATIC TAGS ::= BEGIN
            IMPORTS myConstant, MyType FROM ConstModule { 1 2 3 };

            MyLocalType ::= MyType
        END
    "#;

    let result = Compiler::<RasnBackend, _>::new_with_config(config)
        .add_asn_literal(asn1)
        .compile_to_string()
        .unwrap();

    // Verify constant is imported with correct case (SCREAMING_SNAKE_CASE)
    assert!(
        result.generated.contains("MY_CONSTANT"),
        "Expected constant with SCREAMING_SNAKE_CASE, got:\n{}",
        result.generated
    );

    // Verify type is also imported
    assert!(
        result.generated.contains("MyType"),
        "Expected type to be imported, got:\n{}",
        result.generated
    );
}
