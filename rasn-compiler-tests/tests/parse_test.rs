use rasn_compiler::{prelude::RasnBackend, Compiler, OutputMode};

#[test]
#[ignore]
fn parses_modules() {
    insta::glob!("modules/*", |path| {
        let input = std::fs::read_to_string(path).unwrap();
        let result = Compiler::<RasnBackend, _>::new()
            .add_asn_literal(&input)
            .compile_to_string();

        let output = match result {
            Ok(result) => {
                let mut output = String::new();
                if !result.warnings.is_empty() {
                    output.push_str("Warnings:\n");
                    for warning in &result.warnings {
                        output.push_str(&warning.contextualize(&input));
                        output.push('\n');
                    }
                    output.push_str("\n\n");
                }
                output.push_str("Generated:\n");
                output.push_str(result.generated.trim());
                output.push('\n');
                output
            }
            Err(err) => err.contextualize(&input),
        };

        insta::with_settings!(
            { omit_expression => true, },
            { insta::assert_snapshot!(output); }
        );
    });
}

#[test]
#[ignore]
fn compile_etsi() {
    println!(
        "{:?}",
        Compiler::<RasnBackend, _>::new_with_config(rasn_compiler::prelude::RasnConfig::default())
            // .add_asn_by_path("../rasn-compiler/test_asn1/ngap_class.asn")
            // .add_asn_by_path("../rasn-compiler/test_asn1/ngap_common.asn")
            // .add_asn_by_path("../rasn-compiler/test_asn1/ngap_const.asn")
            // .add_asn_by_path("../rasn-compiler/test_asn1/ngap_container.asn")
            // .add_asn_by_path("../rasn-compiler/test_asn1/ngap_ies.asn")
            // .add_asn_by_path("../rasn-compiler/test_asn1/ngap_pdus.asn")
            .add_asn_by_path("../rasn-compiler/test_asn1/test.asn")
            .set_output_mode(OutputMode::SingleFile("./tests".into()))
            .compile()
    );
}
