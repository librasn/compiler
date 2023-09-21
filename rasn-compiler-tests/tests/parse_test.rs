use rasn_compiler::RasnCompiler;

const UNSUPPORTED: [&'static str; 4] = [
    "./tests/modules/itu-t_h_h350_2006_CommURI.asn1",
    "./tests/modules/itu-t_q_q824.7_2000_Q824-7Asn1Module.asn1",
    "./tests/modules/itu-t_q_q1228_1997_IN-CS2-SCF-SCF-pkgs-contracts-acs.asn1",
    "./tests/modules/itu-t_q_q1238.6_2000_IN-CS3-SCF-SCF-pkgs-contracts-acs.asn1"
];

#[test]
fn parses_modules() {
    for result in std::fs::read_dir("./tests/modules").unwrap() {
        if let Ok(entry) = result {
            let path = entry.path();
            if UNSUPPORTED.contains(&path.to_str().unwrap()) {
                continue;
            }
            RasnCompiler::new()
                .add_asn_by_path(path)
                .compile_to_string()
                .unwrap();
        }
    }
}
