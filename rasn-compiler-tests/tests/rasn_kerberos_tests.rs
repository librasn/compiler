#![cfg_attr(not(test), no_std)]

extern crate alloc;

use rasn::prelude::*;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct ADANDOR {
    #[rasn(tag(explicit(context, 0)))]
    condition_count: Int32,
    #[rasn(tag(explicit(context, 1)))]
    elements: AuthorizationData,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ADIFRELEVANT(pub AuthorizationData);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct ADKDCIssued {
    #[rasn(tag(explicit(context, 0)))]
    ad_checksum: Checksum,
    #[rasn(tag(explicit(context, 1)))]
    i_realm: Option<Realm>,
    #[rasn(tag(explicit(context, 2)))]
    i_sname: Option<PrincipalName>,
    #[rasn(tag(explicit(context, 3)))]
    elements: AuthorizationData,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ADMANDATORYFORKDC(pub AuthorizationData);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(explicit(application, 15)))]
pub struct APREP {
    #[rasn(value("5..=5"), tag(explicit(context, 0)))]
    pvno: u8,
    #[rasn(value("15..=15"), tag(explicit(context, 1)))]
    msg_type: u8,
    #[rasn(tag(explicit(context, 2)))]
    enc_part: EncryptedData,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(explicit(application, 14)))]
pub struct APREQ {
    #[rasn(value("5..=5"), tag(explicit(context, 0)))]
    pvno: u8,
    #[rasn(value("14..=14"), tag(explicit(context, 1)))]
    msg_type: u8,
    #[rasn(tag(explicit(context, 2)))]
    ap_options: APOptions,
    #[rasn(tag(explicit(context, 3)))]
    ticket: Ticket,
    #[rasn(tag(explicit(context, 4)))]
    authenticator: EncryptedData,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct APOptions(pub KerberosFlags);

/// reserved(0),
/// forwardable(1),
/// forwarded(2),
/// proxiable(3),
/// proxy(4),
/// allow-postdate(5),
/// postdated(6),
/// unused7(7),
/// renewable(8),
/// unused9(9),
/// unused10(10),
/// opt-hardware-auth(11),
/// unused12(12),
/// unused13(13),
/// 15 is reserved for canonicalize
/// unused15(15),
/// 26 was unused in 1510
/// disable-transited-check(26),
///
/// renewable-ok(27),
/// enc-tkt-in-skey(28),
/// renew(30),
/// validate(31)

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, tag(explicit(application, 11)))]
pub struct ASREP(pub KDCREP);

/// reserved(0),
/// forwardable(1),
/// forwarded(2),
/// proxiable(3),
/// proxy(4),
/// may-postdate(5),
/// postdated(6),
/// invalid(7),
/// renewable(8),
/// initial(9),
/// pre-authent(10),
/// hw-authent(11),
/// the following are new since 1510
/// transited-policy-checked(12),
/// ok-as-delegate(13)

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, tag(explicit(application, 10)))]
pub struct ASREQ(pub KDCREQ);

/// reserved(0),
/// use-session-key(1),
/// mutual-required(2)
/// Unencrypted authenticator

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(explicit(application, 2)))]
pub struct Authenticator {
    #[rasn(value("5..=5"), tag(explicit(context, 0)))]
    authenticator_vno: u8,
    #[rasn(tag(explicit(context, 1)))]
    crealm: Realm,
    #[rasn(tag(explicit(context, 2)))]
    cname: PrincipalName,
    #[rasn(tag(explicit(context, 3)))]
    cksum: Option<Checksum>,
    #[rasn(tag(explicit(context, 4)))]
    cusec: Microseconds,
    #[rasn(tag(explicit(context, 5)))]
    ctime: KerberosTime,
    #[rasn(tag(explicit(context, 6)))]
    subkey: Option<EncryptionKey>,
    #[rasn(tag(explicit(context, 7)))]
    seq_number: Option<UInt32>,
    #[rasn(tag(explicit(context, 8)))]
    authorization_data: Option<AuthorizationData>,
}

/// Anonymous SEQUENCE OF member

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct AnonymousAuthorizationData {
    #[rasn(tag(context, 0))]
    ad_type: Int32,
    #[rasn(tag(context, 1))]
    ad_data: OctetString,
}

/// NOTE: AuthorizationData is always used as an OPTIONAL field and
/// should not be empty.

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct AuthorizationData(pub SequenceOf<AnonymousAuthorizationData>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct Checksum {
    #[rasn(tag(explicit(context, 0)))]
    cksumtype: Int32,
    #[rasn(tag(explicit(context, 1)))]
    checksum: OctetString,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ETYPEINFO(pub SequenceOf<ETYPEINFOENTRY>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct ETYPEINFOENTRY {
    #[rasn(tag(explicit(context, 0)))]
    etype: Int32,
    #[rasn(tag(explicit(context, 1)))]
    salt: Option<OctetString>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1.."))]
pub struct ETYPEINFO2(pub SequenceOf<ETYPEINFO2ENTRY>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct ETYPEINFO2ENTRY {
    #[rasn(tag(explicit(context, 0)))]
    etype: Int32,
    #[rasn(tag(explicit(context, 1)))]
    salt: Option<KerberosString>,
    #[rasn(tag(explicit(context, 2)))]
    s2kparams: Option<OctetString>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(explicit(application, 27)))]
pub struct EncAPRepPart {
    #[rasn(tag(explicit(context, 0)))]
    ctime: KerberosTime,
    #[rasn(tag(explicit(context, 1)))]
    cusec: Microseconds,
    #[rasn(tag(explicit(context, 2)))]
    subkey: Option<EncryptionKey>,
    #[rasn(tag(explicit(context, 3)))]
    seq_number: Option<UInt32>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, tag(explicit(application, 25)))]
pub struct EncASRepPart(pub EncKDCRepPart);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct EncKDCRepPart {
    #[rasn(tag(explicit(context, 0)))]
    key: EncryptionKey,
    #[rasn(tag(explicit(context, 1)))]
    last_req: LastReq,
    #[rasn(tag(explicit(context, 2)))]
    nonce: UInt32,
    #[rasn(tag(explicit(context, 3)))]
    key_expiration: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 4)))]
    flags: TicketFlags,
    #[rasn(tag(explicit(context, 5)))]
    authtime: KerberosTime,
    #[rasn(tag(explicit(context, 6)))]
    starttime: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 7)))]
    endtime: KerberosTime,
    #[rasn(tag(explicit(context, 8)))]
    renew_till: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 9)))]
    srealm: Realm,
    #[rasn(tag(explicit(context, 10)))]
    sname: PrincipalName,
    #[rasn(tag(explicit(context, 11)))]
    caddr: Option<HostAddresses>,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct EncKrbCredPartTicketInfo(pub SequenceOf<KrbCredInfo>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(explicit(application, 29)))]
pub struct EncKrbCredPart {
    #[rasn(tag(explicit(context, 0)))]
    ticket_info: EncKrbCredPartTicketInfo,
    #[rasn(tag(explicit(context, 1)))]
    nonce: Option<UInt32>,
    #[rasn(tag(explicit(context, 2)))]
    timestamp: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 3)))]
    usec: Option<Microseconds>,
    #[rasn(tag(explicit(context, 4)))]
    s_address: Option<HostAddress>,
    #[rasn(tag(explicit(context, 5)))]
    r_address: Option<HostAddress>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(explicit(application, 28)))]
pub struct EncKrbPrivPart {
    #[rasn(tag(explicit(context, 0)))]
    user_data: OctetString,
    #[rasn(tag(explicit(context, 1)))]
    timestamp: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 2)))]
    usec: Option<Microseconds>,
    #[rasn(tag(explicit(context, 3)))]
    seq_number: Option<UInt32>,
    #[rasn(tag(explicit(context, 4)))]
    s_address: HostAddress,
    #[rasn(tag(explicit(context, 5)))]
    r_address: Option<HostAddress>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, tag(explicit(application, 26)))]
pub struct EncTGSRepPart(pub EncKDCRepPart);

/// Encrypted part of ticket

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(explicit(application, 3)))]
pub struct EncTicketPart {
    #[rasn(tag(explicit(context, 0)))]
    flags: TicketFlags,
    #[rasn(tag(explicit(context, 1)))]
    key: EncryptionKey,
    #[rasn(tag(explicit(context, 2)))]
    crealm: Realm,
    #[rasn(tag(explicit(context, 3)))]
    cname: PrincipalName,
    #[rasn(tag(explicit(context, 4)))]
    transited: TransitedEncoding,
    #[rasn(tag(explicit(context, 5)))]
    authtime: KerberosTime,
    #[rasn(tag(explicit(context, 6)))]
    starttime: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 7)))]
    endtime: KerberosTime,
    #[rasn(tag(explicit(context, 8)))]
    renew_till: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 9)))]
    caddr: Option<HostAddresses>,
    #[rasn(tag(explicit(context, 10)))]
    authorization_data: Option<AuthorizationData>,
}

/// minimum number of bits shall be sent,
/// but no fewer than 32

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct EncryptedData {
    #[rasn(tag(explicit(context, 0)))]
    etype: Int32,
    #[rasn(tag(explicit(context, 1)))]
    kvno: Option<UInt32>,
    #[rasn(tag(explicit(context, 2)))]
    cipher: OctetString,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct EncryptionKey {
    #[rasn(tag(explicit(context, 0)))]
    keytype: Int32,
    #[rasn(tag(explicit(context, 1)))]
    keyvalue: OctetString,
}

/// with no fractional seconds

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct HostAddress {
    #[rasn(tag(explicit(context, 0)))]
    addr_type: Int32,
    #[rasn(tag(explicit(context, 1)))]
    address: OctetString,
}

/// NOTE: HostAddresses is always used as an OPTIONAL field and
/// should not be empty.

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct HostAddresses(pub SequenceOf<HostAddress>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-2147483648..=2147483647"))]
pub struct Int32(pub i32);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct KDCREPPadata(pub SequenceOf<PADATA>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct KDCREP {
    #[rasn(value("5..=5"), tag(explicit(context, 0)))]
    pvno: u8,
    #[rasn(value("11..=13"), tag(explicit(context, 1)))]
    msg_type: u8,
    #[rasn(tag(explicit(context, 2)))]
    padata: Option<KDCREPPadata>,
    #[rasn(tag(explicit(context, 3)))]
    crealm: Realm,
    #[rasn(tag(explicit(context, 4)))]
    cname: PrincipalName,
    #[rasn(tag(explicit(context, 5)))]
    ticket: Ticket,
    #[rasn(tag(explicit(context, 6)))]
    enc_part: EncryptedData,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct KDCREQPadata(pub SequenceOf<PADATA>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct KDCREQ {
    #[rasn(value("5..=5"), tag(explicit(context, 1)))]
    pvno: u8,
    #[rasn(value("10..=12"), tag(explicit(context, 2)))]
    msg_type: u8,
    #[rasn(tag(explicit(context, 3)))]
    padata: Option<KDCREQPadata>,
    #[rasn(tag(explicit(context, 4)))]
    req_body: KDCREQBODY,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct KDCREQBODYEtype(pub SequenceOf<Int32>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct KDCREQBODYAdditionalTickets(pub SequenceOf<Ticket>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct KDCREQBODY {
    #[rasn(tag(explicit(context, 0)))]
    kdc_options: KDCOptions,
    #[rasn(tag(explicit(context, 1)))]
    cname: Option<PrincipalName>,
    #[rasn(tag(explicit(context, 2)))]
    realm: Realm,
    #[rasn(tag(explicit(context, 3)))]
    sname: Option<PrincipalName>,
    #[rasn(tag(explicit(context, 4)))]
    from: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 5)))]
    till: KerberosTime,
    #[rasn(tag(explicit(context, 6)))]
    rtime: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 7)))]
    nonce: UInt32,
    #[rasn(tag(explicit(context, 8)))]
    etype: KDCREQBODYEtype,
    #[rasn(tag(explicit(context, 9)))]
    addresses: Option<HostAddresses>,
    #[rasn(tag(explicit(context, 10)))]
    enc_authorization_data: Option<EncryptedData>,
    #[rasn(tag(explicit(context, 11)))]
    additional_tickets: Option<KDCREQBODYAdditionalTickets>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct KDCOptions(pub KerberosFlags);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct KRBCREDTickets(pub SequenceOf<Ticket>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(explicit(application, 22)))]
pub struct KRBCRED {
    #[rasn(value("5..=5"), tag(explicit(context, 0)))]
    pvno: u8,
    #[rasn(value("22..=22"), tag(explicit(context, 1)))]
    msg_type: u8,
    #[rasn(tag(explicit(context, 2)))]
    tickets: KRBCREDTickets,
    #[rasn(tag(explicit(context, 3)))]
    enc_part: EncryptedData,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(explicit(application, 30)))]
pub struct KRBERROR {
    #[rasn(value("5..=5"), tag(explicit(context, 0)))]
    pvno: u8,
    #[rasn(value("30..=30"), tag(explicit(context, 1)))]
    msg_type: u8,
    #[rasn(tag(explicit(context, 2)))]
    ctime: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 3)))]
    cusec: Option<Microseconds>,
    #[rasn(tag(explicit(context, 4)))]
    stime: KerberosTime,
    #[rasn(tag(explicit(context, 5)))]
    susec: Microseconds,
    #[rasn(tag(explicit(context, 6)))]
    error_code: Int32,
    #[rasn(tag(explicit(context, 7)))]
    crealm: Option<Realm>,
    #[rasn(tag(explicit(context, 8)))]
    cname: Option<PrincipalName>,
    #[rasn(tag(explicit(context, 9)))]
    realm: Realm,
    #[rasn(tag(explicit(context, 10)))]
    sname: PrincipalName,
    #[rasn(tag(explicit(context, 11)))]
    e_text: Option<KerberosString>,
    #[rasn(tag(explicit(context, 12)))]
    e_data: Option<OctetString>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(explicit(application, 21)))]
pub struct KRBPRIV {
    #[rasn(value("5..=5"), tag(explicit(context, 0)))]
    pvno: u8,
    #[rasn(value("21..=21"), tag(explicit(context, 1)))]
    msg_type: u8,
    #[rasn(tag(explicit(context, 3)))]
    enc_part: EncryptedData,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(explicit(application, 20)))]
pub struct KRBSAFE {
    #[rasn(value("5..=5"), tag(explicit(context, 0)))]
    pvno: u8,
    #[rasn(value("20..=20"), tag(explicit(context, 1)))]
    msg_type: u8,
    #[rasn(tag(explicit(context, 2)))]
    safe_body: KRBSAFEBODY,
    #[rasn(tag(explicit(context, 3)))]
    cksum: Checksum,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct KRBSAFEBODY {
    #[rasn(tag(explicit(context, 0)))]
    user_data: OctetString,
    #[rasn(tag(explicit(context, 1)))]
    timestamp: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 2)))]
    usec: Option<Microseconds>,
    #[rasn(tag(explicit(context, 3)))]
    seq_number: Option<UInt32>,
    #[rasn(tag(explicit(context, 4)))]
    s_address: HostAddress,
    #[rasn(tag(explicit(context, 5)))]
    r_address: Option<HostAddress>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("32.."))]
pub struct KerberosFlags(pub BitString);

/// microseconds

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, value("0.."))]
pub struct KerberosString(pub GeneralString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct KerberosTime(pub GeneralizedTime);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct KrbCredInfo {
    #[rasn(tag(explicit(context, 0)))]
    key: EncryptionKey,
    #[rasn(tag(explicit(context, 1)))]
    prealm: Option<Realm>,
    #[rasn(tag(explicit(context, 2)))]
    pname: Option<PrincipalName>,
    #[rasn(tag(explicit(context, 3)))]
    flags: Option<TicketFlags>,
    #[rasn(tag(explicit(context, 4)))]
    authtime: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 5)))]
    starttime: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 6)))]
    endtime: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 7)))]
    renew_till: Option<KerberosTime>,
    #[rasn(tag(explicit(context, 8)))]
    srealm: Option<Realm>,
    #[rasn(tag(explicit(context, 9)))]
    sname: Option<PrincipalName>,
    #[rasn(tag(explicit(context, 10)))]
    caddr: Option<HostAddresses>,
}

/// Anonymous SEQUENCE OF member

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct AnonymousLastReq {
    #[rasn(tag(context, 0))]
    lr_type: Int32,
    #[rasn(tag(context, 1))]
    lr_value: KerberosTime,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct LastReq(pub SequenceOf<AnonymousLastReq>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct METHODDATA(pub SequenceOf<PADATA>);

/// unsigned 32 bit values

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=999999"))]
pub struct Microseconds(pub u32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct PADATA {
    #[rasn(tag(explicit(context, 1)))]
    padata_type: Int32,
    #[rasn(tag(explicit(context, 2)))]
    padata_value: OctetString,
}

/// preauth stuff follows

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct PAENCTIMESTAMP(pub EncryptedData);

/// PA-ENC-TS-ENC

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct PAENCTSENC {
    #[rasn(tag(explicit(context, 0)))]
    patimestamp: KerberosTime,
    #[rasn(tag(explicit(context, 1)))]
    pausec: Option<Microseconds>,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct PrincipalNameNameString(pub SequenceOf<KerberosString>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct PrincipalName {
    #[rasn(tag(explicit(context, 0)))]
    name_type: Int32,
    #[rasn(tag(explicit(context, 1)))]
    name_string: PrincipalNameNameString,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Realm(pub KerberosString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, tag(explicit(application, 13)))]
pub struct TGSREP(pub KDCREP);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, tag(explicit(application, 12)))]
pub struct TGSREQ(pub KDCREQ);

/// Anonymous SEQUENCE OF member

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct AnonymousTYPEDDATA {
    #[rasn(tag(context, 0))]
    data_type: Int32,
    #[rasn(tag(context, 1))]
    data_value: Option<OctetString>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1.."))]
pub struct TYPEDDATA(pub SequenceOf<AnonymousTYPEDDATA>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(explicit(application, 1)))]
pub struct Ticket {
    #[rasn(value("5..=5"), tag(explicit(context, 0)))]
    tkt_vno: u8,
    #[rasn(tag(explicit(context, 1)))]
    realm: Realm,
    #[rasn(tag(explicit(context, 2)))]
    sname: PrincipalName,
    #[rasn(tag(explicit(context, 3)))]
    enc_part: EncryptedData,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct TicketFlags(pub KerberosFlags);

/// encoded Transited field

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct TransitedEncoding {
    #[rasn(tag(explicit(context, 0)))]
    tr_type: Int32,
    #[rasn(tag(explicit(context, 1)))]
    contents: OctetString,
}

/// signed values representable in 32 bits

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4294967295"))]
pub struct UInt32(pub u32);

/// OID arc for KerberosV5
///
/// This OID may be used to identify Kerberos protocol messages
/// encapsulated in other protocols.
///
/// This OID also designates the OID arc for KerberosV5-related OIDs.
///
/// NOTE: RFC 1510 had an incorrect value (5) for "dod" in its OID.

pub const ID_KRB5: &Oid = Oid::const_new(&[1, 3, 6, 1, 5, 2]);

#[test]
fn as_rep() {
    let as_rep = ASREP(KDCREP {
        pvno: 5,
        msg_type: 11,
        padata: Some(KDCREPPadata(vec![PADATA {
            padata_type: Int32(19),
            padata_value: OctetString::from_static(&[
                0x30, 0x1d, 0x30, 0x1b, 0xa0, 0x03, 0x02, 0x01, 0x12, 0xa1, 0x14, 0x1b, 0x12, 0x43,
                0x4f, 0x4d, 0x50, 0x41, 0x4e, 0x59, 0x2e, 0x49, 0x4e, 0x54, 0x75, 0x73, 0x65, 0x72,
            ]),
        }])),
        crealm: Realm(KerberosString(
            GeneralString::try_from("COMPANY.INT".to_string()).unwrap(),
        )),
        cname: PrincipalName {
            name_type: Int32(1),
            name_string: PrincipalNameNameString(vec![KerberosString(
                GeneralString::try_from(String::from("user")).unwrap(),
            )]),
        },
        ticket: Ticket {
            tkt_vno: 5,
            realm: Realm(KerberosString(
                GeneralString::try_from("COMPANY.INT".to_string()).unwrap(),
            )),
            sname: PrincipalName {
                name_type: Int32(2),
                name_string: PrincipalNameNameString(vec![
                    KerberosString(GeneralString::try_from(String::from("krbtgt")).unwrap()),
                    KerberosString(GeneralString::try_from(String::from("COMPANY.INT")).unwrap()),
                ]),
            },
            enc_part: EncryptedData {
                etype: Int32(18),
                kvno: Some(UInt32(2)),
                cipher: OctetString::from_static(&[0xde, 0xad, 0xbe, 0xef]),
            },
        },
        enc_part: EncryptedData {
            etype: Int32(18),
            kvno: Some(UInt32(13)),
            cipher: OctetString::from_static(&[0xde, 0xad, 0xbe, 0xef]),
        },
    });
    let data: &[u8] = &[
        0x6b, 0x81, 0xc2, 0x30, 0x81, 0xbf, 0xa0, 0x03, 0x02, 0x01, 0x05, 0xa1, 0x03, 0x02, 0x01,
        0x0b, 0xa2, 0x29, 0x30, 0x27, 0x30, 0x25, 0xa1, 0x03, 0x02, 0x01, 0x13, 0xa2, 0x1e, 0x04,
        0x1c, 0x30, 0x1d, 0x30, 0x1b, 0xa0, 0x03, 0x02, 0x01, 0x12, 0xa1, 0x14, 0x1b, 0x12, 0x43,
        0x4f, 0x4d, 0x50, 0x41, 0x4e, 0x59, 0x2e, 0x49, 0x4e, 0x54, 0x75, 0x73, 0x65, 0x72, 0xa3,
        0x0d, 0x1b, 0x0b, 0x43, 0x4f, 0x4d, 0x50, 0x41, 0x4e, 0x59, 0x2e, 0x49, 0x4e, 0x54, 0xa4,
        0x11, 0x30, 0x0f, 0xa0, 0x03, 0x02, 0x01, 0x01, 0xa1, 0x08, 0x30, 0x06, 0x1b, 0x04, 0x75,
        0x73, 0x65, 0x72, 0xa5, 0x50, 0x61, 0x4e, 0x30, 0x4c, 0xa0, 0x03, 0x02, 0x01, 0x05, 0xa1,
        0x0d, 0x1b, 0x0b, 0x43, 0x4f, 0x4d, 0x50, 0x41, 0x4e, 0x59, 0x2e, 0x49, 0x4e, 0x54, 0xa2,
        0x20, 0x30, 0x1e, 0xa0, 0x03, 0x02, 0x01, 0x02, 0xa1, 0x17, 0x30, 0x15, 0x1b, 0x06, 0x6b,
        0x72, 0x62, 0x74, 0x67, 0x74, 0x1b, 0x0b, 0x43, 0x4f, 0x4d, 0x50, 0x41, 0x4e, 0x59, 0x2e,
        0x49, 0x4e, 0x54, 0xa3, 0x14, 0x30, 0x12, 0xa0, 0x03, 0x02, 0x01, 0x12, 0xa1, 0x03, 0x02,
        0x01, 0x02, 0xa2, 0x06, 0x04, 0x04, 0xde, 0xad, 0xbe, 0xef, 0xa6, 0x14, 0x30, 0x12, 0xa0,
        0x03, 0x02, 0x01, 0x12, 0xa1, 0x03, 0x02, 0x01, 0x0d, 0xa2, 0x06, 0x04, 0x04, 0xde, 0xad,
        0xbe, 0xef,
    ];

    let enc = rasn::der::encode(&as_rep).unwrap();
    assert_eq!(data, &enc);
    assert_eq!(as_rep, rasn::der::decode(&enc).unwrap());
}

#[test]
fn as_req() {
    let as_req = ASREQ(KDCREQ {
        pvno: 5,
        msg_type: 10,
        padata: Some(KDCREQPadata(vec![PADATA {
            padata_type: Int32(128),
            padata_value: OctetString::from_static(&[0x30, 0x05, 0xa0, 0x03, 0x01, 0x01, 0xff]),
        }])),
        req_body: KDCREQBODY {
            kdc_options: KDCOptions(KerberosFlags(BitString::from_slice(&[
                0x40, 0x81, 0x00, 0x10,
            ]))),
            cname: Some(PrincipalName {
                name_type: Int32(1),
                name_string: PrincipalNameNameString(vec![KerberosString(
                    GeneralString::try_from(String::from("user")).unwrap(),
                )]),
            }),
            realm: Realm(KerberosString(
                GeneralString::try_from("COMPANY.INT".to_string()).unwrap(),
            )),
            sname: Some(PrincipalName {
                name_type: Int32(2),
                name_string: PrincipalNameNameString(vec![
                    KerberosString(GeneralString::try_from(String::from("krbtgt")).unwrap()),
                    KerberosString(GeneralString::try_from(String::from("COMPANY.INT")).unwrap()),
                ]),
            }),
            from: None,
            till: KerberosTime(
                GeneralizedTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap(),
            ),
            rtime: Some(KerberosTime(
                GeneralizedTime::parse_from_rfc3339("2052-03-04T11:11:11Z").unwrap(),
            )),
            nonce: UInt32(12345678),
            etype: KDCREQBODYEtype(vec![
                Int32(18),
                Int32(23),
                Int32(-133),
                Int32(-128),
                Int32(24),
                Int32(-135),
            ]),
            addresses: Some(HostAddresses(vec![HostAddress {
                addr_type: Int32(20),
                address: OctetString::from("CLIENT01        "),
            }])),
            enc_authorization_data: None,
            additional_tickets: None,
        },
    });

    let data: &[u8] = &[
        0x6a, 0x81, 0xdc, 0x30, 0x81, 0xd9, 0xa1, 0x03, 0x02, 0x01, 0x05, 0xa2, 0x03, 0x02, 0x01,
        0x0a, 0xa3, 0x15, 0x30, 0x13, 0x30, 0x11, 0xa1, 0x04, 0x02, 0x02, 0x00, 0x80, 0xa2, 0x09,
        0x04, 0x07, 0x30, 0x05, 0xa0, 0x03, 0x01, 0x01, 0xff, 0xa4, 0x81, 0xb5, 0x30, 0x81, 0xb2,
        0xa0, 0x07, 0x03, 0x05, 0x00, 0x40, 0x81, 0x00, 0x10, 0xa1, 0x11, 0x30, 0x0f, 0xa0, 0x03,
        0x02, 0x01, 0x01, 0xa1, 0x08, 0x30, 0x06, 0x1b, 0x04, 0x75, 0x73, 0x65, 0x72, 0xa2, 0x0d,
        0x1b, 0x0b, 0x43, 0x4f, 0x4d, 0x50, 0x41, 0x4e, 0x59, 0x2e, 0x49, 0x4e, 0x54, 0xa3, 0x20,
        0x30, 0x1e, 0xa0, 0x03, 0x02, 0x01, 0x02, 0xa1, 0x17, 0x30, 0x15, 0x1b, 0x06, 0x6b, 0x72,
        0x62, 0x74, 0x67, 0x74, 0x1b, 0x0b, 0x43, 0x4f, 0x4d, 0x50, 0x41, 0x4e, 0x59, 0x2e, 0x49,
        0x4e, 0x54, 0xa5, 0x11, 0x18, 0x0f, 0x31, 0x39, 0x37, 0x30, 0x30, 0x31, 0x30, 0x31, 0x30,
        0x30, 0x30, 0x30, 0x30, 0x30, 0x5a, 0xa6, 0x11, 0x18, 0x0f, 0x32, 0x30, 0x35, 0x32, 0x30,
        0x33, 0x30, 0x34, 0x31, 0x31, 0x31, 0x31, 0x31, 0x31, 0x5a, 0xa7, 0x06, 0x02, 0x04, 0x00,
        0xbc, 0x61, 0x4e, 0xa8, 0x16, 0x30, 0x14, 0x02, 0x01, 0x12, 0x02, 0x01, 0x17, 0x02, 0x02,
        0xff, 0x7b, 0x02, 0x01, 0x80, 0x02, 0x01, 0x18, 0x02, 0x02, 0xff, 0x79, 0xa9, 0x1d, 0x30,
        0x1b, 0x30, 0x19, 0xa0, 0x03, 0x02, 0x01, 0x14, 0xa1, 0x12, 0x04, 0x10, 0x43, 0x4c, 0x49,
        0x45, 0x4e, 0x54, 0x30, 0x31, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    ];

    let enc = rasn::der::encode(&as_req).unwrap();
    assert_eq!(data, enc);
    assert_eq!(as_req, rasn::der::decode(&enc).unwrap());
}
