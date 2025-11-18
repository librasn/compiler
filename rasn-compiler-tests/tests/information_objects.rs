use rasn_compiler_tests::e2e_pdu;

// This suite contains tests to test x681 Information Object Classes and its
// associated notations
e2e_pdu!(
    information_object,
    rasn_compiler::prelude::RasnConfig {
        opaque_open_types: false,
        ..Default::default()
    },
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
              { IDENTIFIED BY asn-val-unknown-order} |
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
