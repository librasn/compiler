use bnf::Grammar;

#[test]
fn fuzz() {
    let asn1_bnf = std::fs::read("./tests/fuzz_grammar/asn1.bnf").unwrap();
    let grammar: Grammar = String::from_utf8(asn1_bnf).unwrap().parse().unwrap();
    println!("{:#?}", grammar.generate().unwrap())
}
