use rasn_compiler::{prelude::RasnBackend, Compiler, OutputMode};

#[test]
#[ignore]
fn parses_modules() {
    let read_dir = std::fs::read_dir("./tests/modules").unwrap();
    let mut errors = String::new();
    let mut succeeded = 0;
    let mut failed = 0;
    for entry in read_dir.flatten() {
        let path = entry.path();
        println!("{:?}", &path);
        if let Err(e) = Compiler::<RasnBackend, _>::new()
            .add_asn_by_path(path.clone())
            .compile_to_string()
        {
            failed += 1;
            errors.push_str(&format!(
                r#"
----------------------------------------------------------------------------
{path:?}
----------------------------------------------------------------------------
{}

                    "#,
                e.contextualize(&std::fs::read_to_string(path.clone()).unwrap())
            ))
        } else {
            succeeded += 1;
        }
    }
    let success_rate = 100 * succeeded / (succeeded + failed);
    std::fs::write(
        "./parse_test_results.txt",
        format!(
            r#"
Success rate of {success_rate}%.
Parsed {succeeded} ASN1 modules without running into unrecoverable errors.
Failed to parse {failed} modules with the following errors:

    "#
        ) + &errors,
    )
    .unwrap();
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
