use rasn_compiler::prelude::*;

fn compile(source: &str) -> String {
    let result = Compiler::<RasnBackend, _>::new()
        .add_asn_literal(source)
        .compile_to_string()
        .expect("compile ASN.1");
    assert!(result.warnings.is_empty(), "{:?}", result.warnings);
    result.generated.split_whitespace().collect()
}

#[test]
fn nested_parameterized_lists_keep_size_bounds() {
    let output = compile(
        r#"
        Lists DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        Bounded {INTEGER: lower, INTEGER: upper} ::= SEQUENCE (SIZE(lower..upper)) OF BOOLEAN
        AForwarded {INTEGER: upper} ::= Bounded {1, upper}
        Concrete ::= AForwarded {8}
        END
    "#,
    );
    assert!(output.contains("size(\"1..=8\")"), "{output}");
}

#[test]
fn parameterized_templates_are_not_imported_as_rust_items() {
    let output = compile(
        r#"
        Templates DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        SetupRelease {T} ::= CHOICE {release NULL, setup T}
        END
        Uses DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        IMPORTS SetupRelease{} FROM Templates;
        Concrete ::= SetupRelease {BOOLEAN}
        END
    "#,
    );
    assert!(!output.contains("usesuper::templates"), "{output}");
}

#[test]
fn resolved_class_fields_import_their_named_types() {
    let output = compile(
        r#"
        Common DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        Identifier ::= CHOICE {local INTEGER(0..65535), global OBJECT IDENTIFIER}
        END
        Templates DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        IMPORTS Identifier FROM Common;
        IE ::= CLASS {&id Identifier UNIQUE, &Type} WITH SYNTAX {ID &id TYPE &Type}
        Field {IE: Set} ::= SEQUENCE {id IE.&id ({Set}), value IE.&Type ({Set}{@id})}
        END
        Uses DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        IMPORTS IE, Field{} FROM Templates;
        Set IE ::= {...}
        Concrete ::= Field {{Set}}
        Zzz ::= BOOLEAN
        END
    "#,
    );
    let uses = output.split("pubmoduses{").nth(1).expect("Uses module");
    assert!(
        uses.contains("usesuper::common::Identifier")
            || uses.contains("usesuper::common::{Identifier}"),
        "{uses}"
    );
    assert!(uses.contains("pubid:Identifier"), "{uses}");
}

#[test]
fn colliding_type_names_keep_distinct_references_and_imports() {
    let output = compile(
        r#"
        Lists DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        ECGIList ::= SEQUENCE (SIZE(1..8)) OF BOOLEAN
        ECGI-List ::= SEQUENCE (SIZE(1..256)) OF INTEGER
        ECGIList2 ::= BOOLEAN
        Pair ::= SEQUENCE {first ECGIList, second ECGI-List}
        END
        Uses DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        IMPORTS ECGI-List FROM Lists;
        Foreign ::= ECGI-List
        END
    "#,
    );
    assert!(output.contains("pubstructECGIList("), "{output}");
    assert!(output.contains("pubstructECGIList3("), "{output}");
    assert!(
        output.contains("pubfirst:ECGIList,pubsecond:ECGIList3"),
        "{output}"
    );
    let uses = output.split("pubmoduses{").nth(1).expect("Uses module");
    assert!(
        uses.contains("usesuper::lists::{ECGIList3}")
            || uses.contains("usesuper::lists::ECGIList3"),
        "{uses}"
    );
    assert!(uses.contains("pubstructForeign(pubECGIList3)"), "{uses}");
}

#[test]
fn forwarded_formals_shadow_module_values_in_either_declaration_order() {
    for forwarded in ["AForwarded", "ZForwarded"] {
        let output = compile(&format!(
            r#"
            Lists DEFINITIONS AUTOMATIC TAGS ::= BEGIN
            upper INTEGER ::= 100
            Bounded {{INTEGER: lower, INTEGER: upper}} ::= SEQUENCE (SIZE(lower..upper)) OF BOOLEAN
            {forwarded} {{INTEGER: upper}} ::= Bounded {{1, upper}}
            Concrete ::= {forwarded} {{8}}
            END
        "#
        ));
        assert!(output.contains("size(\"1..=8\")"), "{output}");
        assert!(!output.contains("size(\"1..=100\")"), "{output}");
    }
}

#[test]
fn qualified_references_do_not_create_unqualified_imports() {
    let output = compile(
        r#"
        A DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        Foo-Bar ::= INTEGER
        END
        B DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        FooBar ::= BOOLEAN
        Foreign ::= A.Foo-Bar
        END
    "#,
    );
    let module = output.split("pubmodb{").nth(1).unwrap();
    assert!(!module.contains("usesuper::a::"), "{module}");
    assert!(
        module.contains("pubstructForeign(pubsuper::a::FooBar)"),
        "{module}"
    );
}

#[test]
fn renamed_sequences_use_their_own_default_helpers() {
    let output = compile(
        r#"
        A DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        FooBar ::= SEQUENCE {value INTEGER (0..255) DEFAULT 2}
        Foo-Bar ::= SEQUENCE {value INTEGER (0..255) DEFAULT 1}
        END
    "#,
    );
    assert!(
        output.contains("value:foo_bar2_value_default()"),
        "{output}"
    );
    assert!(
        output.contains("fnfoo_bar2_value_default()->u8{1}"),
        "{output}"
    );
}

#[test]
fn imported_names_reserve_the_receiving_modules_namespace() {
    let output = compile(
        r#"
        A DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        FooBar ::= BOOLEAN
        Foo-Bar ::= INTEGER
        END
        B DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        IMPORTS Foo-Bar FROM A;
        FooBar2 ::= BOOLEAN
        Foreign ::= Foo-Bar
        END
    "#,
    );
    let module = output.split("pubmodb{").nth(1).unwrap();
    assert!(module.contains("FooBar2asFooBar22"), "{module}");
    assert!(module.contains("pubstructForeign(pubFooBar22)"), "{module}");
}

#[test]
fn forwarded_type_parameters_are_resolved_in_the_caller_scope() {
    let output = compile(
        r#"
        A DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        Wrapper {T} ::= SEQUENCE {v T}
        Forward {T} ::= Wrapper {T}
        Concrete ::= Forward {BOOLEAN}
        Pair {T,U} ::= SEQUENCE {a T, b U}
        Swapped {X,Y} ::= Pair {Y,X}
        Reordered ::= Swapped {BOOLEAN, INTEGER}
        END
    "#,
    );
    assert!(output.contains("pubv:bool"), "{output}");
    assert!(output.contains("puba:Integer,pubb:bool"), "{output}");
}

#[test]
fn templates_link_components_when_instantiated() {
    let output = compile(
        r#"
        A DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        Base ::= SEQUENCE {a INTEGER}
        Wrapper {T} ::= SEQUENCE {b T, COMPONENTS OF Base}
        Concrete ::= Wrapper {BOOLEAN}
        END
    "#,
    );
    assert!(output.contains("pubb:bool,puba:Integer"), "{output}");
}

#[test]
fn qualified_types_inside_templates_do_not_bind_formal_parameters() {
    let output = compile(
        r#"
        A DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        T ::= INTEGER
        END
        B DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        Wrapper {T} ::= SEQUENCE {v A.T, w T}
        Concrete ::= Wrapper {BOOLEAN}
        END
    "#,
    );
    let module = output.split("pubmodb{").nth(1).unwrap();
    assert!(module.contains("pubv:super::a::T,pubw:bool"), "{module}");
    assert!(!module.contains("usesuper::a::"), "{module}");
}
