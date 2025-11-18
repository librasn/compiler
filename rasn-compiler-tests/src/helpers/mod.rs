use std::num::ParseIntError;
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[macro_export]
macro_rules! e2e_pdu {
    ($suite:ident, $asn1:literal) => {
        e2e_pdu!($suite, rasn_compiler::prelude::RasnConfig::default(), $asn1);
    };
    ($suite:ident, $config:expr, $asn1:literal) => {
        #[test]
        fn $suite() {
            rasn_compiler_derive::asn1!($asn1);

            let input = format!(
                "TestModule DEFINITIONS AUTOMATIC TAGS::= BEGIN {} END",
                $asn1
            );

            let result = rasn_compiler::Compiler::<rasn_compiler::prelude::RasnBackend, _>::new_with_config(
                $config,
            )
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
                {
                    description => $asn1,
                    omit_expression => true,
                },
                { insta::assert_snapshot!(output); }
            )
        }
    };
}

#[macro_export]
macro_rules! e2e_module {
    ($suite:ident, $asn1:literal, $expected:literal) => {
        #[test]
        fn $suite() {
            assert_eq!(
                rasn_compiler::Compiler::new()
                    .add_asn_literal(asn1)
                    .compile_to_string()
                    .unwrap()
                    .0
                    .replace(|c: char| c.is_whitespace(), ""),
                $expected
                    .to_string()
                    .replace(|c: char| c.is_whitespace(), ""),
            )
        }
    };
}
