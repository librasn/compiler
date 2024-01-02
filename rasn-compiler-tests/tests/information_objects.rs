use rasn::types::Any;
/// This suite contains tests to test x681 Information Object Classes and its
/// associated notations
use rasn_compiler_derive::asn1;

#[test]
fn information_object_class() {
    #[allow(non_camel_case_types)]
    asn1!(
        r#"
          ERROR ::= CLASS {
            &errorCode ErrorCode UNIQUE,
            &ParameterType
          } WITH SYNTAX { [&ParameterType] IDENTIFIED BY &errorCode }

          ErrorCode ::= CHOICE {
                local INTEGER,
                global OBJECT IDENTIFIER
            }

          Errors ERROR ::= {
              { BOOLEAN IDENTIFIED BY asn-val-security-failure} |
              { INTEGER IDENTIFIED BY asn-val-unknown-branch} |
              { IDENTIFIED BY asn-val-unkn  own-order} |
              { BIT STRING (SIZE(4)) IDENTIFIED BY local: 4},
              ...
          }

          Actual ::= SEQUENCE {
            errorCode ERROR.&errorCode ({Errors}),
            parameter ERROR.&ParameterType ({Errors}{@errorCode}) OPTIONAL
          }

          asn-val-security-failure ErrorCode ::= local: 1
          asn-val-unknown-branch ErrorCode ::= local: 2
          asn-val-unknown-order ErrorCode ::= local: 3 
          "#
    );

    let instance = asn1::Actual {
        error_code: asn1::ASN_VAL_UNKNOWN_ORDER,
        parameter: None, 
    };

    assert_eq!(rasn::uper::encode(&instance).unwrap(), vec![56, 64, 0, 0, 0, 0, 0, 0, 1, 128])
}
