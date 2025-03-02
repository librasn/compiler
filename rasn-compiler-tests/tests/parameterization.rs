use rasn_compiler_tests::e2e_pdu;

e2e_pdu! {
parameterized_type,
r#"
        ParamType { INTEGER: lower, BOOLEAN: flag } ::= SEQUENCE {
            int-value INTEGER (lower..12),
            bool-value BOOLEAN DEFAULT flag
        }
        ImplType ::= ParamType { 2, TRUE }
    "#,
r#"
        #[derive (AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn (automatic_tags)]
        pub struct ImplType {
            #[rasn(value("2..=12"), identifier = "int-value")]
            pub int_value : u8,
            #[rasn (default = "impl_type_bool_value_default", identifier = "bool-value")]
            pub bool_value : bool,
        }

        impl ImplType {
            pub fn new (int_value: u8 , bool_value: bool) -> Self {
                Self { int_value , bool_value, }
            }
        }

        fn impl_type_bool_value_default () -> bool {
            true
        }
    "#
}

e2e_pdu! {
    parameterized_information_object_classes,
    rasn_compiler::prelude::RasnConfig {
        opaque_open_types: false,
        ..Default::default()
    },
    r#"
        NGAP-PROTOCOL-EXTENSION ::= CLASS {
            &id				INTEGER			UNIQUE,
            &criticality	INTEGER,
            &Extension,
            &presence		BOOLEAN
        }
        WITH SYNTAX {
            ID				&id
            CRITICALITY		&criticality
            EXTENSION		&Extension
            PRESENCE		&presence
        }

        ProtocolExtensionContainer {NGAP-PROTOCOL-EXTENSION : ExtensionSetParam} ::= 
            SEQUENCE (SIZE (1..maxProtocolExtensions)) OF
            ProtocolExtensionField {{ExtensionSetParam}}

        ProtocolExtensionField {NGAP-PROTOCOL-EXTENSION : ExtensionSetParam} ::= SEQUENCE {
            id					NGAP-PROTOCOL-EXTENSION.&id				({ExtensionSetParam}),
            criticality			NGAP-PROTOCOL-EXTENSION.&criticality	({ExtensionSetParam}{@id}),
            extensionValue		NGAP-PROTOCOL-EXTENSION.&Extension		({ExtensionSetParam}{@id})
        }

        A2X-PC5-FlowBitRates ::= SEQUENCE {
            a2X-GuaranteedFlowBitRate		BOOLEAN,
            iE-Extensions		ProtocolExtensionContainer { {A2X-PC5-FlowBitRates-ExtIEs} }	OPTIONAL,
            ...
        }

        A2X-PC5-FlowBitRates-ExtIEs NGAP-PROTOCOL-EXTENSION ::= {
            ...
        }
    "#,
    r#"
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousA2XPC5FlowBitRatesIEExtensions {
        pub id: Integer,
        pub criticality: Integer,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl AnonymousA2XPC5FlowBitRatesIEExtensions {
        pub fn new(id: Integer, criticality: Integer, extension_value: Any) -> Self {
            Self {
                id,
                criticality,
                extension_value,
            }
        }
    }
    impl AnonymousA2XPC5FlowBitRatesIEExtensions {
        pub fn decode_extension_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<A2XPC5FlowBitRatesExtIEs_Extension, D::Error> {
            A2XPC5FlowBitRatesExtIEs_Extension::decode(
                decoder,
                Some(&self.extension_value),
                &self.id,
            )
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1.."))]
    pub struct A2XPC5FlowBitRatesIEExtensions(
        pub SequenceOf<AnonymousA2XPC5FlowBitRatesIEExtensions>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "A2X-PC5-FlowBitRates")]
    #[non_exhaustive]
    pub struct A2XPC5FlowBitRates {
        #[rasn(identifier = "a2X-GuaranteedFlowBitRate")]
        pub a2_x__guaranteed_flow_bit_rate: bool,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e__extensions: Option<A2XPC5FlowBitRatesIEExtensions>,
    }
    impl A2XPC5FlowBitRates {
        pub fn new(
            a2_x__guaranteed_flow_bit_rate: bool,
            i_e__extensions: Option<A2XPC5FlowBitRatesIEExtensions>,
        ) -> Self {
            Self {
                a2_x__guaranteed_flow_bit_rate,
                i_e__extensions,
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum A2XPC5FlowBitRatesExtIEs_Extension {}
    impl A2XPC5FlowBitRatesExtIEs_Extension {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &Integer,
        ) -> Result<Self, D::Error> {
            match identifier {
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
            identifier: &Integer,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
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
    #[derive(Debug, Clone, PartialEq)]
    pub enum A2XPC5FlowBitRatesExtIEs_criticality {}
    impl A2XPC5FlowBitRatesExtIEs_criticality {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &Integer,
        ) -> Result<Self, D::Error> {
            match identifier {
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
            identifier: &Integer,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
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
    #[derive(Debug, Clone, PartialEq)]
    pub enum A2XPC5FlowBitRatesExtIEs_id {}
    impl A2XPC5FlowBitRatesExtIEs_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &Integer,
        ) -> Result<Self, D::Error> {
            match identifier {
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
            identifier: &Integer,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
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
    #[derive(Debug, Clone, PartialEq)]
    pub enum A2XPC5FlowBitRatesExtIEs_presence {}
    impl A2XPC5FlowBitRatesExtIEs_presence {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &Integer,
        ) -> Result<Self, D::Error> {
            match identifier {
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
            identifier: &Integer,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
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
    "#
}

e2e_pdu! {
anonymous_type_param,
r#"
            SetupRelease { ElementTypeParam } ::= CHOICE { 
                release         NULL,
                setup           ElementTypeParam 
            }

            LocationMeasurementInfo ::= SEQUENCE {
                test BOOLEAN
            }

            LocationMeasurementIndication-IEs ::=       SEQUENCE { 
                measurementIndication      SetupRelease { LocationMeasurementInfo }, 
                lateNonCriticalExtension   OCTET STRING OPTIONAL,
                nonCriticalExtension       SEQUENCE{} OPTIONAL 
            }
        "#,
    r#"
            #[doc = "Inner type"]
            #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
            #[rasn(choice, automatic_tags)]
            pub enum LocationMeasurementIndicationIEsMeasurementIndication {
                release(()),
                setup(LocationMeasurementInfo),
            }
            
            #[doc = "Inner type"]
            #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
            #[rasn(automatic_tags)]
            pub struct LocationMeasurementIndicationIEsNonCriticalExtension {}
            
            impl LocationMeasurementIndicationIEsNonCriticalExtension {
                pub fn new() -> Self {
                    Self {} 
                }
            }
            
            #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
            #[rasn(automatic_tags, identifier = "LocationMeasurementIndication-IEs")]
            pub struct LocationMeasurementIndicationIEs {
                #[rasn(identifier = "measurementIndication")]
                pub measurement_indication: LocationMeasurementIndicationIEsMeasurementIndication,
                #[rasn(identifier = "lateNonCriticalExtension")]
                pub late_non_critical_extension: Option<OctetString>,
                #[rasn(identifier = "nonCriticalExtension")]
                pub non_critical_extension: Option<LocationMeasurementIndicationIEsNonCriticalExtension>,
            }
            
            impl LocationMeasurementIndicationIEs {
                pub fn new(
                    measurement_indication: LocationMeasurementIndicationIEsMeasurementIndication,
                    late_non_critical_extension: Option<OctetString>,
                    non_critical_extension: Option<LocationMeasurementIndicationIEsNonCriticalExtension>,
                ) -> Self {
                    Self {
                        measurement_indication,
                        late_non_critical_extension,
                        non_critical_extension,
                    }
                }
            }
            
            #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
            #[rasn(automatic_tags)]
            pub struct LocationMeasurementInfo { 
                pub test: bool,
            }
            
            impl LocationMeasurementInfo {
                pub fn new(test: bool) -> Self {
                    Self { test }
                }
            }
        "#
}
