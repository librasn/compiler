use rasn_compiler_tests::e2e_pdu;


#[test]
fn t() {
    println!(
        "{}",
        rasn_compiler::Compiler::new()
            .add_asn_literal(
                r#" 
        TestModule DEFINITIONS AUTOMATIC TAGS ::= BEGIN

        value SEQUENCE OF INTEGER ::= { 1, 2, 3 }

        END
        "#
            )
            .compile_to_string()
            .unwrap().generated
    )
}

// e2e_pdu!(
//     sequence_of_primitive_value,
//     r#" value SEQUENCE OF INTEGER ::= { 1, 2, 3 }"#,
//     r#" lazy_static! {
//             pub static ref VALUE: Vec<Integer> = alloc::vec![Integer::from(1), Integer::from(2), Integer::from(3)]; 
//         }                                                       "#
// );