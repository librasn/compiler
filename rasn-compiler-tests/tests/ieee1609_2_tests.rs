#![recursion_limit = "32"]
use bitvec::prelude::*;
use rasn_compiler_tests::{helpers::*, asn1::ieee1609_2::*};

use rasn::prelude::*;

#[test]
fn decodes_secured_header() {
    let mut encoder = rasn::uper::Encoder::new(
        rasn::uper::enc::EncoderOptions::unaligned(),
    );

    let signature = Ieee1609Dot2Data {
        protocol_version: Uint8(3),
        content: Ieee1609Dot2Content::SignedData(Box::new(SignedData {
            hash_id: HashAlgorithm::Sha256,
            tbs_data: ToBeSignedData {
                payload: SignedDataPayload::new(
                    Some(Ieee1609Dot2Data {
                        protocol_version: Uint8(3),
                        content: Ieee1609Dot2Content::UnsecuredData(Opaque(OctetString::try_from(decode_hex("20500280003b010014001e0ddf3f5b7da06f53ac1c81287c07c5e02f801409f60000000007d100000202df3f5b7d53ac405a44c22f8e61f645e1b41b400047f190489f7fa00aa4bfe9ea8333ff01fffa00283300001bfc28fe4f34bc7fff00").unwrap()).unwrap()))
                    }),
                    None,
                    None,
                ),
                header_info: HeaderInfo::new(
                    Psid(36.into()),
                    Some(Time64(Uint64(621166941188830))),
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(SequenceOfHashedId3(vec![HashedId3(OctetString::try_from(decode_hex("7ce7f9").unwrap()).unwrap())])),
                    None,
                    None,
                    None,
                ),
            },
            signer: SignerIdentifier::Certificate(SequenceOfCertificate(vec![Certificate(CertificateBase {
                version: Uint8(3),
                r_type: CertificateType::Explicit,
                issuer: IssuerIdentifier::Sha256AndDigest(HashedId8(OctetString::try_from(decode_hex("5d5dcbeefbe7d22d").unwrap()).unwrap())),
                to_be_signed: ToBeSignedCertificate::new(
                    CertificateId::None(()),
                    HashedId3(OctetString::from(vec![0,0,0])),
                    CrlSeries(Uint16(0)),
                    ValidityPeriod {
                        start: Time32(Uint32(616377605)),
                        duration: Duration::Years(Uint16(1))
                    },
                    None,
                    Some(SubjectAssurance(OctetString::from(vec![224]))),
                    Some(SequenceOfPsidSsp(vec![
                        PsidSsp {
                            psid: Psid(36.into()),
                            ssp: Some(ServiceSpecificPermissions::BitmapSsp(BitmapSsp(OctetString::try_from(decode_hex("").unwrap()).unwrap()))),
                        },
                        PsidSsp {
                            psid: Psid(37.into()),
                            ssp: Some(ServiceSpecificPermissions::BitmapSsp(BitmapSsp(OctetString::try_from(decode_hex("").unwrap()).unwrap()))),
                        },
                        PsidSsp {
                            psid: Psid(140.into()),
                            ssp: Some(ServiceSpecificPermissions::BitmapSsp(BitmapSsp(OctetString::try_from(decode_hex("").unwrap()).unwrap()))),
                        },
                        PsidSsp {
                            psid: Psid(141.into()),
                            ssp: None,
                        },
                        PsidSsp {
                            psid: Psid(638.into()),
                            ssp: Some(ServiceSpecificPermissions::BitmapSsp(BitmapSsp(OctetString::try_from(decode_hex("").unwrap()).unwrap()))),
                        },
                        PsidSsp {
                            psid: Psid(639.into()),
                            ssp: Some(ServiceSpecificPermissions::BitmapSsp(BitmapSsp(OctetString::try_from(decode_hex("").unwrap()).unwrap()))),
                        },
                        PsidSsp {
                            psid: Psid(1023.into()),
                            ssp: None,
                        }
                    ])),
                    None,
                    None,
                    None,
                    None,
                    VerificationKeyIndicator::VerificationKey(PublicVerificationKey::EcdsaNistP256(EccP256CurvePoint::CompressedY0(OctetString::try_from(decode_hex("dea08ea8e83e46244a8f98a1df151e938d2639acdaa410804880aa362e855dad").unwrap()).unwrap()))),
                    None,
                    ),
                signature: Some(Signature::EcdsaBrainpoolP256r1Signature(EcdsaP256Signature {
                    r_sig: EccP256CurvePoint::CompressedY1(OctetString::try_from(decode_hex("5bd800fce37f7070dff59027a39d19ae8de9607612cbb2309af5fe8943300802").unwrap()).unwrap()),
                    s_sig: OctetString::try_from(decode_hex("8e294ff7efaecabf824cab932704cb982080f342900c1fda11f6da434005ed85").unwrap()).unwrap()
                })),
            })])),
            signature: Signature::EcdsaNistP256Signature(EcdsaP256Signature {
                r_sig: EccP256CurvePoint::CompressedY0(OctetString::try_from(decode_hex("2f0e6198a844841fe9f39d5a03fd41121ecb45d6d6d6c548da1e55b0bc377cfa").unwrap()).unwrap()),
                s_sig: OctetString::try_from(decode_hex("4c08bc73c2820943302a81124fcf139ea47e41ee6eb6cde71e0813185998abd5").unwrap()).unwrap()
            }),
        }))
    };
    signature.encode(&mut encoder).unwrap();
    let encoded = encoder.output();
    println!("{:?}", encoded);

    let header = decode_hex("0381004003805f20500280003b010014001e0ddf3f5b7da06f53ac1c81287c07c5e02f801409f60000000007d100000202df3f5b7d53ac405a44c22f8e61f645e1b41b400047f190489f7fa00aa4bfe9ea8333ff01fffa00283300001bfc28fe4f34bc7fff00c00124000234f2b2e032de0205800501017ce7f9810101800300805d5dcbeefbe7d22d3083000000000024bd2d05860001e0010780012481040301fffc80012581050401ffffff80018c81050402ffffe000018d8002027e810201018002027f81020101000203ff808082dea08ea8e83e46244a8f98a1df151e938d2639acdaa410804880aa362e855dad81835bd800fce37f7070dff59027a39d19ae8de9607612cbb2309af5fe89433008028e294ff7efaecabf824cab932704cb982080f342900c1fda11f6da434005ed8580822f0e6198a844841fe9f39d5a03fd41121ecb45d6d6d6c548da1e55b0bc377cfa4c08bc73c2820943302a81124fcf139ea47e41ee6eb6cde71e0813185998abd5").unwrap();
    let mut decoder = rasn::uper::Decoder::new(
        header.view_bits::<Msb0>(),
        rasn::uper::de::DecoderOptions::unaligned(),
    );
    println!("{:?}", Ieee1609Dot2Data::decode(&mut decoder).unwrap());
}