use rasn_compiler_tests::e2e_pdu;

/// This suite contains tests to test x681 Information Object Classes and its
/// associated notations

e2e_pdu!(
    information_object,
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
          "#,
    r#"
          #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct Actual {
        pub error_code: ErrorCode,
        pub parameter: Option<Any>,
    }
    impl Actual {
        pub fn new(error_code: ErrorCode, parameter: Option<Any>) -> Self {
            Self {
                error_code,
                parameter,
            }
        }
    }
    impl Actual {
        pub fn decode_parameter<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<Errors_ParameterType, D::Error> {
            Errors_ParameterType::decode(decoder, self.parameter.as_ref(), &self.error_code)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq)]
    #[rasn(choice, automatic_tags)]
    pub enum ErrorCode {
        local(Integer),
        global(ObjectIdentifier),
    }
    #[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
    #[rasn(size("4"), delegate)]
    pub struct Inner_Errors_ParameterType_2(pub BitString);
    #[derive(Debug, Clone, PartialEq)]
    pub enum Errors_ParameterType {
        AsnValSecurityFailure(bool),
        AsnValUnknownBranch(Integer),
        Errors_ParameterType_2(Inner_Errors_ParameterType_2),
    }
    impl Errors_ParameterType {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &ErrorCode,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &*ASN_VAL_SECURITY_FAILURE => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AsnValSecurityFailure)?),
                i if i == &*ASN_VAL_UNKNOWN_BRANCH => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AsnValUnknownBranch)?),
                i if i == &ErrorCode::local(Integer::from(4)) => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::Errors_ParameterType_2)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &ErrorCode,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AsnValSecurityFailure(inner), i) if i == &*ASN_VAL_SECURITY_FAILURE => {
                    inner.encode(encoder)
                }
                (Self::AsnValUnknownBranch(inner), i) if i == &*ASN_VAL_UNKNOWN_BRANCH => {
                    inner.encode(encoder)
                }
                (Self::Errors_ParameterType_2(inner), i)
                    if i == &ErrorCode::local(Integer::from(4)) =>
                {
                    inner.encode(encoder)
                }
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    lazy_static! {
        pub static ref ASN_VAL_SECURITY_FAILURE: ErrorCode = ErrorCode::local(Integer::from(1));
    }
    lazy_static! {
        pub static ref ASN_VAL_UNKNOWN_BRANCH: ErrorCode = ErrorCode::local(Integer::from(2));
    }
    lazy_static! {
        pub static ref ASN_VAL_UNKNOWN_ORDER: ErrorCode = ErrorCode::local(Integer::from(3));
    }
          "#
);
