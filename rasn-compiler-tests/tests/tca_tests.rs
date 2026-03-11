use std::path::PathBuf;

use rasn_compiler::{prelude::RasnBackend, Compiler};

fn modules_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/modules")
}

fn compile_snapshot(module_filenames: &[&str]) -> String {
    let dir = modules_dir();
    let paths: Vec<PathBuf> = module_filenames.iter().map(|f| dir.join(f)).collect();

    let result = Compiler::<RasnBackend, _>::new()
        .add_asn_sources_by_path(paths.into_iter())
        .compile_to_string();

    match result {
        Ok(result) => {
            let mut output = String::new();
            if !result.warnings.is_empty() {
                output.push_str("Warnings:\n");
                for warning in &result.warnings {
                    output.push_str(&format!("  {warning}\n"));
                }
                output.push('\n');
            }
            output.push_str("Generated:\n");
            output.push_str(result.generated.trim());
            output.push('\n');
            output
        }
        Err(err) => format!("Error: {err}\n"),
    }
}

/// TCA Profile Package v2.3.1 — standalone module (no external imports).
/// Covers: AUTOMATIC TAGS, CHOICE (30 variants), SEQUENCE with OPTIONAL/DEFAULT,
/// SEQUENCE OF CHOICE (File type), APPLICATION/PRIVATE tag classes, NULL OPTIONAL flags.
#[test]
fn tca_v2_3_1_compiles() {
    let output = compile_snapshot(&["tca_PEDefinitions_v2.3.1.asn1"]);
    insta::with_settings!(
        { omit_expression => true },
        { insta::assert_snapshot!(output); }
    );
}

/// TCA Profile Package v3.4.1 — requires PKIX1Explicit88 for `Certificate` import.
/// Adds over v2.3.1: PE-IoT, PE-SSIM, PE-SSIM-EAPTLSParameters, PE-DF-SNPN,
/// PE-DF-5GPROSE, IotOptions in ProfileHeader, SSIM service flags.
#[test]
fn tca_v3_4_1_compiles() {
    let output = compile_snapshot(&[
        "ietf_rfc_rfc3280_PKIX1Explicit88.asn1",
        "tca_PEDefinitions_v3.4.1.asn1",
    ]);
    insta::with_settings!(
        { omit_expression => true },
        { insta::assert_snapshot!(output); }
    );
}
