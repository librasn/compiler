use rasn::AsnType;
use rasn_compiler_derive::asn1;

mod forwarded_formal_bounds {
    use super::*;

    asn1!(
        r#"
        Lists DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        upper INTEGER ::= 100
        Bounded {INTEGER: lower, INTEGER: upper} ::= SEQUENCE (SIZE(lower..upper)) OF BOOLEAN
        ZForwarded {INTEGER: upper} ::= Bounded {1, upper}
        Concrete ::= ZForwarded {8}
        END
    "#
    );
    #[test]
    fn do_not_capture_a_module_value() {
        let size = lists::Concrete::CONSTRAINTS.size().unwrap().constraint;
        assert!(size.contains(&8));
        assert!(!size.contains(&9));
    }
}

mod colliding_sequences {
    use super::*;

    asn1!(
        r#"
        A DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        FooBar ::= SEQUENCE {value INTEGER (0..255) DEFAULT 2}
        Foo-Bar ::= SEQUENCE {value INTEGER (0..255) DEFAULT 1}
        END
    "#
    );
    #[test]
    fn keep_distinct_default_values() {
        assert_eq!(a::FooBar::default().value, 2);
        assert_eq!(a::FooBar2::default().value, 1);
    }
}

mod qualified_references {
    use super::*;

    asn1!(
        r#"
        A DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        Foo-Bar ::= INTEGER
        END
        B DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        FooBar ::= BOOLEAN
        Foreign ::= A.Foo-Bar
        END
    "#
    );
    #[test]
    fn do_not_import_conflicting_local_names() {
        let value = b::Foreign(a::FooBar(7.into()));
        assert_eq!(value.0 .0, 7.into());
        assert!(b::FooBar(true).0);
    }
}

mod imported_collision_suffixes {
    use super::*;

    asn1!(
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
    "#
    );
    #[test]
    fn do_not_shadow_local_declarations() {
        let value = b::Foreign(a::FooBar2(7.into()));
        assert_eq!(value.0 .0, 7.into());
        assert!(b::FooBar2(true).0);
    }
}

mod forwarded_type_arguments {
    use super::*;

    asn1!(
        r#"
        A DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        Wrapper {T} ::= SEQUENCE {v T}
        Forward {T} ::= Wrapper {T}
        Concrete ::= Forward {BOOLEAN}
        Pair {T,U} ::= SEQUENCE {a T, b U}
        Swapped {X,Y} ::= Pair {Y,X}
        Reordered ::= Swapped {BOOLEAN, INTEGER}
        END
    "#
    );
    #[test]
    fn resolve_arguments_before_binding_callee_formals() {
        let value = a::Concrete::new(true);
        assert_eq!(rasn::aper::encode(&value).unwrap(), [0x80]);
        let reordered = a::Reordered::new(5.into(), true);
        assert_eq!(reordered.a, 5.into());
        assert!(reordered.b);
    }
}

mod template_components {
    use super::*;
    use rasn::Decoder;

    asn1!(
        r#"
        A DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        Base ::= SEQUENCE {a INTEGER}
        Wrapper {T} ::= SEQUENCE {b T, COMPONENTS OF Base}
        Concrete ::= Wrapper {BOOLEAN}
        END
    "#
    );

    #[derive(rasn::AsnType, rasn::Encode, rasn::Decode)]
    #[rasn(automatic_tags)]
    struct Reference {
        b: bool,
        a: rasn::types::Integer,
    }

    #[test]
    fn retain_components_and_their_wire_encoding() {
        let value = a::Concrete::new(true, 5.into());
        let expected = rasn::aper::encode(&Reference {
            b: true,
            a: 5.into(),
        })
        .unwrap();
        assert_eq!(expected, [0x80, 0x01, 0x05]);
        assert_eq!(rasn::aper::encode(&value).unwrap(), expected);
        let decoded: a::Concrete = rasn::aper::decode(&expected).unwrap();
        assert!(decoded.b);
        assert_eq!(decoded.a, 5.into());
    }
}

mod qualified_formal_parameters {
    use super::*;

    asn1!(
        r#"
        A DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        T ::= INTEGER
        END
        B DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        Wrapper {T} ::= SEQUENCE {v A.T, w T}
        Concrete ::= Wrapper {BOOLEAN}
        END
    "#
    );
    #[test]
    fn retain_qualified_module_types() {
        let value = b::Concrete::new(a::T(5.into()), true);
        assert_eq!(value.v.0, 5.into());
        assert!(value.w);
        assert_eq!(rasn::aper::encode(&value).unwrap(), [0x01, 0x05, 0x80]);
    }
}
