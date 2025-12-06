#![allow(non_camel_case_types)]
use rasn_compiler_tests::e2e_pdu;

e2e_pdu!(boolean, "Test-Boolean ::= BOOLEAN");

e2e_pdu!(boolean_value, "test-boolean BOOLEAN ::= TRUE");

e2e_pdu!(integer, "Test-Int ::= INTEGER");

e2e_pdu!(integer_value, "test-int INTEGER ::= 4");

e2e_pdu!(integer_u32_max, "test-int INTEGER ::= 4294967295");

e2e_pdu!(integer_value_constrained, "test-int INTEGER(0..255) ::= 4");

e2e_pdu!(
    integer_value_large_constrained,
    "test-int INTEGER(0..MAX) ::= 4"
);

e2e_pdu!(
    integer_distinguished_values,
    "Test-Int ::= INTEGER {
        first(0),
        second(1),
        third (2)
    } (0..2)"
);

e2e_pdu!(
    integer_const,
    r#" Test-Int ::= INTEGER
        test-int-val Test-Int ::= 4"#
);

e2e_pdu!(integer_strict, "Test-Int ::= INTEGER (4)");

e2e_pdu!(integer_strict_ext, "Test-Int ::= INTEGER (4,...)");

e2e_pdu!(
    integer_strict_ext_const,
    r#" Test-Int ::= INTEGER (4,...)
        test-int-val Test-Int ::= 4"#
);

e2e_pdu!(
    integer_range,
    r#" Test-Int ::= INTEGER (4..6)
        test-int-val Test-Int ::= 5"#
);

e2e_pdu!(
    integer_range_ext_const,
    r#" Test-Int ::= INTEGER (4..6,...)
        test-int-val Test-Int ::= 5"#
);

e2e_pdu!(null, "Test-Int ::= NULL");

e2e_pdu!(null_value, "test-null NULL ::= NULL");

e2e_pdu!(
    bit_string,
    r#" Test-Bits ::= BIT STRING
        test-bits-val Test-Bits ::= '10101010'B"#
);

e2e_pdu!(bit_string_value, "test-bits BIT STRING ::= '1010'B");

e2e_pdu!(bit_string_value_hex, "test-bits BIT STRING ::= 'FF'H");

e2e_pdu!(
    bit_string_named_bits,
    r#" Test-Bits ::= BIT STRING {
            first(0) -- first bit --,
            last(1),
        } SIZE(2)                                           "#
);

e2e_pdu!(bit_string_strict, "Test-Bits ::= BIT STRING SIZE(4)");

e2e_pdu!(
    bit_string_strict_ext,
    "Test-Bits ::= BIT STRING (SIZE(4,...))"
);

e2e_pdu!(
    bit_string_range,
    r#" Test-Bits ::= BIT STRING (SIZE(4..6))
        test-bits-val Test-Bits ::= '10101'B"#
);

e2e_pdu!(
    bit_string_range_ext,
    r#" Test-Bits ::= BIT STRING (SIZE(4..6,...))
        test-bits-val Test-Bits ::= 'D5'H"#
);

e2e_pdu!(
    octet_string,
    r#" Test-Octets ::= OCTET STRING
        test-octets-val Test-Octets ::= '10101010'B"#
);

e2e_pdu!(octet_string_value, "test-bytes OCTET STRING ::= 'FF'H");

e2e_pdu!(
    octet_string_value_binary,
    "test-bytes OCTET STRING ::= '11111111'B"
);

e2e_pdu!(octet_string_strict, "Test-Octets ::= OCTET STRING SIZE(4)");

e2e_pdu!(
    octet_string_strict_ext,
    "Test-Octets ::= OCTET STRING (SIZE(4,...))"
);

e2e_pdu!(
    octet_string_range,
    r#" Test-Octets ::= OCTET STRING (SIZE(4..6))
        test-octets-val Test-Octets ::= 'FF010201FF'H"#
);

e2e_pdu!(
    octet_string_range_ext,
    r#" Test-Octets ::= OCTET STRING SIZE(4..6,...)
        test-octets-val Test-Octets ::= 'FF010201FF2EDD60'H"#
);

e2e_pdu!(
    enumerated,
    r#" Test-Enum ::= ENUMERATED {
            test-1,
            test-2(7)
        }
        test-enum-val Test-Enum ::= test-2          "#
);

e2e_pdu!(
    enumerated_negative,
    r#" Test-Enum ::= ENUMERATED {
            test-1(3),
            test-2(-7)
        }
        test-enum-val Test-Enum ::= test-2          "#
);

e2e_pdu!(
    empty_enumerated,
    r#" Test-Enum ::= ENUMERATED {
        }                           "#
);

e2e_pdu!(
    extended_enumerated,
    r#" Test-Enum ::= ENUMERATED {
            test-1,
            ...,
            test-2(7),
        }
        test-enum-val Test-Enum ::= test-2          "#
);

// REAL Types are currently not supported by rasn

e2e_pdu!(
    bmp,
    r#" Test-String ::= BMPString
        test-string-val Test-String ::= "012345""#
);
e2e_pdu!(
    bmp_strict,
    r#" Test-String ::= BMPString SIZE (4)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    bmp_strict_ext,
    r#" Test-String ::= BMPString SIZE (4,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    bmp_range,
    r#" Test-String ::= BMPString SIZE (4..6)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    bmp_range_ext,
    r#" Test-String ::= BMPString SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    numeric,
    r#" Test-String ::= NumericString
        test-string-val Test-String ::= "012345""#
);
e2e_pdu!(
    numeric_strict,
    r#" Test-String ::= NumericString SIZE (4)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    numeric_strict_ext,
    r#" Test-String ::= NumericString SIZE (4,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    numeric_range,
    r#" Test-String ::= NumericString SIZE (4..6)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    numeric_range_ext,
    r#" Test-String ::= NumericString SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    ia5,
    r#" Test-String ::= IA5String
        test-string-val Test-String ::= "012345""#
);
e2e_pdu!(
    ia5_strict,
    r#" Test-String ::= IA5String SIZE (4)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    ia5_strict_ext,
    r#" Test-String ::= IA5String SIZE (4,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    ia5_range,
    r#" Test-String ::= IA5String SIZE (4..6)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    ia5_range_ext,
    r#" Test-String ::= IA5String SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    printable,
    r#" Test-String ::= PrintableString
        test-string-val Test-String ::= "012345""#
);
e2e_pdu!(
    printable_strict,
    r#" Test-String ::= PrintableString SIZE (4)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    printable_strict_ext,
    r#" Test-String ::= PrintableString SIZE (4,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    printable_range,
    r#" Test-String ::= PrintableString SIZE (4..6)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    printable_range_ext,
    r#" Test-String ::= PrintableString SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    general,
    r#" Test-String ::= GeneralString
        test-string-val Test-String ::= "012345""#
);
e2e_pdu!(
    general_strict,
    r#" Test-String ::= GeneralString SIZE (4)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    general_strict_ext,
    r#" Test-String ::= GeneralString SIZE (4,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    general_range,
    r#" Test-String ::= GeneralString SIZE (4..6)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    general_range_ext,
    r#" Test-String ::= GeneralString SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    graphic,
    r#" Test-String ::= GraphicString
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    graphic_strict,
    r#" Test-String ::= GraphicString SIZE (4)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    graphic_strict_ext,
    r#" Test-String ::= GraphicString SIZE (4,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    graphic_range,
    r#" Test-String ::= GraphicString SIZE (4..6)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    graphic_range_ext,
    r#" Test-String ::= GraphicString SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    utf8,
    r#" Test-String ::= UTF8String
        test-string-val Test-String ::= "012345""#
);
e2e_pdu!(
    utf8_strict,
    r#" Test-String ::= UTF8String SIZE (4)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    utf8_strict_ext,
    r#" Test-String ::= UTF8String SIZE (4,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    utf8_range,
    r#" Test-String ::= UTF8String SIZE (4..6)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    utf8_range_ext,
    r#" Test-String ::= UTF8String SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    visible,
    r#" Test-String ::= VisibleString
        test-string-val Test-String ::= "012345""#
);
e2e_pdu!(
    visible_strict,
    r#" Test-String ::= VisibleString SIZE (4)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    visible_strict_ext,
    r#" Test-String ::= VisibleString SIZE (4,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    visible_range,
    r#" Test-String ::= VisibleString SIZE (4..6)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    visible_range_ext,
    r#" Test-String ::= VisibleString SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#
);

e2e_pdu!(
    visible_from,
    r#" FQDN ::= VisibleString(FROM ("a".."z" | "A".."Z" | "0".."9" | ".-")) (SIZE (1..255))"#
);

e2e_pdu!(
    oid_value,
    r#"organizationRoot OBJECT IDENTIFIER ::= { iso(1) identified-organization(3) teletrust(36) gematik(15) organization(2) legal-entity(3) type(1) }"#
);

e2e_pdu!(
    oid_value_with_well_known_references,
    r#"organizationRoot OBJECT IDENTIFIER ::= { iso identified-organization teletrust(36) gematik(15) organization(2) legal-entity(3) type(1) }"#
);

e2e_pdu!(
    oid_value_with_local_references,
    r#"
        organizationRoot OBJECT IDENTIFIER ::= { iso identified-organization teletrust(36) gematik(15) organization(2) legal-entity(3) type(1) }
        oidMedicalPractice OBJECT IDENTIFIER ::= { organizationRoot medicalPractice(2) }
    "#
);
