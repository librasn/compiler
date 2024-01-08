use rasn_compiler::RasnCompiler;

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
        if let Err(e) = RasnCompiler::new()
            .add_asn_by_path(path.clone())
            .compile_to_string()
        {
            failed += 1;
            errors.push_str(&format!(
                r#"
----------------------------------------------------------------------------
{path:?}
----------------------------------------------------------------------------
{e:#?}

                    "#
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
        RasnCompiler::new()
            .add_asn_by_path("../rasn-compiler/test_asn1/v2x.asn")
            //.add_asn_by_path("./tests/modules/itu-t_x_x501_2001_EnhancedSecurity.asn1")
            .set_output_path("./src")
            .compile()
    );
}
