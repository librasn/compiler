#[allow(unused_imports)]
use super::*;

#[test]
fn link_enumerated_value() {
    let default_tltd = ToplevelTypeDefinition {
        name: String::new(),
        ty: ASN1Type::Enumerated(Enumerated {
            members: vec![Enumeral {
                // Use the same enum member for all ENUMERATED
                name: "member".to_owned(),
                description: None,
                index: 0,
            }],
            extensible: None,
            constraints: Vec::new(),
        }),
        comments: String::new(),
        tag: None,
        parameterization: None,
        module_header: None,
    };
    let tlds = vec![
        ToplevelDefinition::Type(ToplevelTypeDefinition {
            name: "Enum1".to_owned(),
            ..default_tltd.clone()
        }),
        ToplevelDefinition::Type(ToplevelTypeDefinition {
            name: "Enum2".to_owned(),
            ..default_tltd.clone()
        }),
        ToplevelDefinition::Type(ToplevelTypeDefinition {
            name: "Enum3".to_owned(),
            ..default_tltd
        }),
        // A value with a reference to the second enums member
        ToplevelDefinition::Value(ToplevelValueDefinition {
            name: "enum-ref".to_owned(),
            value: ASN1Value::ElsewhereDeclaredValue {
                identifier: "member".to_owned(),
                module: None,
                parent: None,
            },
            associated_type: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                identifier: "Enum2".to_owned(),
                parent: None,
                module: None,
                constraints: Vec::new(),
            }),
            parameterization: None,
            comments: String::new(),
            module_header: None,
        }),
    ];
    let validator = Validator::new(tlds);

    let (result, warnings) = validator.validate().unwrap();

    assert_eq!(warnings, []);
    let enum_ref = result.iter().find(|tld| tld.name() == "enum-ref").unwrap();
    assert_eq!(
        enum_ref,
        &ToplevelDefinition::Value(ToplevelValueDefinition {
            name: "enum-ref".to_owned(),
            associated_type: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                identifier: "Enum2".to_owned(),
                parent: None,
                module: None,
                constraints: Vec::new(),
            }),
            value: ASN1Value::EnumeratedValue {
                // Found on the correct ENUMERATED
                enumerated: "Enum2".to_owned(),
                enumerable: "member".to_owned(),
            },
            comments: String::new(),
            parameterization: None,
            module_header: None
        }),
    );
}
