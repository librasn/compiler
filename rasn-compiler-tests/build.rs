use std::path::PathBuf;

use rasn_compiler::{prelude::RasnBackend, Compiler};

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let modules = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("tests/modules");

    // TCA v2.3.1 — standalone, no external imports.
    let result = Compiler::<RasnBackend, _>::new()
        .add_asn_by_path(modules.join("tca_PEDefinitions_v2.3.1.asn1"))
        .compile_to_string()
        .expect("rasn-compiler failed on TCA v2.3.1");
    std::fs::write(out_dir.join("tca_v2_3_1.rs"), &result.generated).unwrap();

    // TCA v3.4.1 — imports Certificate from PKIX1Explicit88.
    // Both modules are compiled together; the generated pedefinitions module
    // references pkix1_explicit88 as a sibling module.
    let result = Compiler::<RasnBackend, _>::new()
        .add_asn_by_path(modules.join("ietf_rfc_rfc3280_PKIX1Explicit88.asn1"))
        .add_asn_by_path(modules.join("tca_PEDefinitions_v3.4.1.asn1"))
        .compile_to_string()
        .expect("rasn-compiler failed on TCA v3.4.1 + PKIX1Explicit88");
    std::fs::write(out_dir.join("tca_v3_4_1.rs"), &result.generated).unwrap();

    println!("cargo:rerun-if-changed=tests/modules/tca_PEDefinitions_v2.3.1.asn1");
    println!("cargo:rerun-if-changed=tests/modules/tca_PEDefinitions_v3.4.1.asn1");
    println!(
        "cargo:rerun-if-changed=tests/modules/ietf_rfc_rfc3280_PKIX1Explicit88.asn1"
    );
}
