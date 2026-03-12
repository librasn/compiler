// Stage-2 compile tests for TCA Profile Package.
//
// The build.rs pre-compiles both TCA versions using rasn-compiler and writes
// the generated Rust to OUT_DIR.  Including those files here means rustc
// must compile the generated code — any type error in the output is a
// compile-time failure of this test binary.
//
// KNOWN FAILURE (issue #167.2): tca_v2_3_1 currently fails to compile.
//   peakaparameter_sqn_init_default() returns SequenceOf<OctetString> but the
//   field type is PEAKAParameterSqnInit.  Remove this comment when fixed.

mod tca_v2_3_1 {
    include!(concat!(env!("OUT_DIR"), "/tca_v2_3_1.rs"));
}

mod tca_v3_4_1 {
    include!(concat!(env!("OUT_DIR"), "/tca_v3_4_1.rs"));
}

#[test]
fn tca_v2_3_1_generated_rust_is_valid() {
    // Compilation of mod tca_v2_3_1 above is the real assertion.
    let _ = std::any::type_name::<tca_v2_3_1::pedefinitions::ProfileElement>();
}

#[test]
fn tca_v3_4_1_generated_rust_is_valid() {
    // Compilation of mod tca_v3_4_1 above is the real assertion.
    let _ = std::any::type_name::<tca_v3_4_1::pedefinitions::ProfileElement>();
}
