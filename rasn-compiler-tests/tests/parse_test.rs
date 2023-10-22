use rasn_compiler::RasnCompiler;

#[test]
fn parses_modules() {
    let read_dir = std::fs::read_dir("./tests/modules").unwrap();
    let mut errors = String::new();
    let mut succeeded = 0;
    let mut failed = 0;
    for result in read_dir {
        if let Ok(entry) = result {
            let path = entry.path();
            println!("{:?}", &path);
            if let Err(e) = RasnCompiler::new()
                .add_asn_by_path(path.clone())
                .compile_to_string() {
                    failed += 1;
                    errors.push_str(&format!(r#"
----------------------------------------------------------------------------
{path:?}
----------------------------------------------------------------------------
{e:#?}

                    "#))
                } else {
                    succeeded += 1;
                }
        }
    }
    let success_rate = 100 * succeeded / (succeeded + failed);
    std::fs::write("./parse_test_results.txt", format!(r#"
Success rate of {success_rate}%. 
Parsed {succeeded} ASN1 modules without running into unrecoverable errors.
Failed to parse {failed} modules with the following errors:
    
    "#) + &errors).unwrap();
}

#[test]
fn compile_etsi() {
    RasnCompiler::new()
        .add_asn_by_path("./tests/modules/v2x_cam.asn1")
        .set_output_path("./cam.rs")
        .compile();
}