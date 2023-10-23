use super::utils::join_annotations;

pub fn rasn_imports_and_generic_types(include_file_headers: bool) -> String {
    format!(
        r#"{}

        extern crate alloc;
        
        use rasn::prelude::*;"#,
        if include_file_headers {
            "#![no_std]"
        } else {
            ""
        }
    )
}

pub fn typealias_template(
    comments: String,
    name: String,
    alias: String,
    tag_annotations: String,
    constraint_annotations: String,
) -> String {
    let rasn_annotations: String = join_annotations(vec![
        "delegate".into(),
        tag_annotations,
        constraint_annotations,
    ]);
    format!(
        r#"
{comments}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
{rasn_annotations}pub struct {name}(pub {alias});
"#
    )
}

pub fn object_identifier_value_template(comments: String, name: String, value: String) -> String {
    format!(
        r#"{comments}
pub const {name}: &'static Oid = &Oid::const_new(&{value});
"#
    )
}

pub fn integer_value_template(
    comments: String,
    name: String,
    vtype: &str,
    value: String,
) -> String {
    format!(
        r#"{comments}
pub const {name}: {vtype} = {value};
"#
    )
}

pub fn integer_template(
    comments: String,
    name: String,
    constraint_annotations: String,
    tag_annotations: String,
    integer_type: String,
) -> String {
    let rasn_annotations: String = join_annotations(vec![
        "delegate".into(),
        tag_annotations,
        constraint_annotations,
    ]);
    format!(
        r#"
{comments}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
{rasn_annotations}pub struct {name}(pub {integer_type});
"#
    )
}

pub fn time_value_template(
    comments: String,
    name: String,
    type_name: String,
    stringified_value: String,
) -> String {
    format!(
        r#"
{comments}
pub const {name}: {type_name} = {stringified_value};
"#
    )
}

pub fn generalized_time_template(
    comments: String,
    name: String,
    tag_annotations: String,
) -> String {
    let rasn_annotations: String = join_annotations(vec![
        "delegate".into(),
        tag_annotations,
    ]);
    format!(
        r#"
{comments}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
{rasn_annotations}pub struct {name}(pub GeneralizedTime);
"#
    )
}

pub fn utc_time_template(
    comments: String,
    name: String,
    tag_annotations: String,
) -> String {
    let rasn_annotations: String = join_annotations(vec![
        "delegate".into(),
        tag_annotations,
    ]);
    format!(
        r#"
{comments}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
{rasn_annotations}pub struct {name}(pub UtcTime);
"#
    )
}

pub fn bit_string_template(
    comments: String,
    name: String,
    constraint_annotations: String,
    tag_annotations: String,
) -> String {
    let rasn_annotations: String = join_annotations(vec![
        "delegate".into(),
        tag_annotations,
        constraint_annotations,
    ]);
    format!(
        r#"
{comments}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
{rasn_annotations}pub struct {name}(pub BitString);
"#
    )
}

pub fn octet_string_template(
    comments: String,
    name: String,
    constraint_annotations: String,
    tag_annotations: String,
) -> String {
    let rasn_annotations: String = join_annotations(vec![
        "delegate".into(),
        tag_annotations,
        constraint_annotations,
    ]);
    format!(
        r#"
{comments}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
{rasn_annotations}pub struct {name}(pub OctetString);
"#
    )
}

pub fn char_string_template(
    comments: String,
    name: String,
    string_type: String,
    constraint_annotations: String,
    alphabet_annotations: String,
    tag_annotations: String,
) -> String {
    let rasn_annotations: String = join_annotations(vec![
        "delegate".into(),
        tag_annotations,
        constraint_annotations,
        alphabet_annotations,
    ]);
    format!(
        r#"
{comments}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
{rasn_annotations}pub struct {name}(pub {string_type});
"#
    )
}

pub fn boolean_template(comments: String, name: String, tag_annotations: String) -> String {
    let rasn_annotations: String = join_annotations(vec!["delegate".into(), tag_annotations]);
    format!(
        r#"
{comments}
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
{rasn_annotations}pub struct {name}(pub bool);
"#
    )
}

pub fn null_value_template(comments: String, name: String) -> String {
    format!(
        r#"{comments}
pub const {name} = ();
"#
    )
}

pub fn null_template(comments: String, name: String, tag_annotations: String) -> String {
    let rasn_annotations: String = join_annotations(vec!["delegate".into(), tag_annotations]);
    format!(
        r#"
{comments}
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
{rasn_annotations}pub struct {name}(());
"#
    )
}

pub fn any_template(comments: String, name: String, tag_annotations: String) -> String {
    let rasn_annotations: String = join_annotations(vec!["delegate".into(), tag_annotations]);
    format!(
        r#"
{comments}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
{rasn_annotations}pub struct {name}(Any);
"#
    )
}

pub fn oid_template(comments: String, name: String, tag_annotations: String, constraint_annotations: String) -> String {
    let rasn_annotations: String = join_annotations(vec!["delegate".into(), tag_annotations, constraint_annotations]);
    format!(
        r#"
{comments}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
{rasn_annotations}pub struct {name}(pub ObjectIdentifier);
"#
    )
}

pub fn enumerated_template(
    comments: String,
    name: String,
    extensible: &str,
    enum_members: String,
    tag_annotations: String,
) -> String {
    let rasn_annotations = join_annotations(vec!["enumerated".into(), tag_annotations]);
    format!(
        r#"
{comments}
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
{rasn_annotations}{extensible}
pub enum {name} {{
    {enum_members}
}}
"#
    )
}

pub fn sequence_or_set_template(
    comments: String,
    name: String,
    extensible: &str,
    members: String,
    nested_members: String,
    tag_annotations: String,
    set_annotation: String,
    default_methods: String,
    new_impl: String
) -> String {
    let rasn_annotations = join_annotations(vec![set_annotation, tag_annotations]);
    format!(
        r#"
        {nested_members}
        {comments}
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        {rasn_annotations}{extensible}pub struct {name} {{
            {members}
        }}

        {new_impl}

        {default_methods}"#
    )
}

pub fn sequence_or_set_of_template(
    is_set_of: bool,
    comments: String,
    name: String,
    anonymous_item: String,
    member_type: String,
    constraint_annotations: String,
    tag_annotations: String,
) -> String {
    let rasn_annotations: String = join_annotations(vec![
        "delegate".into(),
        tag_annotations,
        constraint_annotations,
    ]);
    format!(
        r#"
        {anonymous_item}
{comments}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
{rasn_annotations}pub struct {name}(pub {}Of<{member_type}>);
"#,
if is_set_of { "Set" } else { "Sequence" }
    )
}

pub fn choice_template(
    comments: String,
    name: String,
    extensible: &str,
    options: String,
    nested_options: String,
    tag_annotations: String,
) -> String {
    let rasn_annotations = join_annotations(vec!["choice".into(), tag_annotations]);
    format!(
        r#"{nested_options}
{comments}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
{rasn_annotations}{extensible}pub enum {name} {{
  {options}
}}
"#,
    )
}

// pub fn information_object_class_template(
//     comments: String,
//     name: String,
//     information_object_class_descriptor: String,
// ) -> String {
//     format!(
//         r#"{comments}
// pub trait {name} {{
//   fn descriptor() -> InformationObjectClass {{
//     {information_object_class_descriptor}
//   }}
// }}
// "#
//     )
// }

// pub fn information_object_template(
//     comments: String,
//     derive: &str,
//     inner_members: String,
//     name: String,
//     member_declaration: String,
//     extension_decl: String,
//     decode_member_body: String,
//     extension_decoder: String,
//     information_object_descriptor: String,
// ) -> String {
//     format!(
//         r#"
// {inner_members}

// {comments}{derive}
// pub struct {name} {{
// {member_declaration}{extension_decl}
// }}

// impl<'a, I: AsBytes + Debug + 'a> DecodeMember<'a, I> for {name} {{
// fn decode_member_at_index<D>(&mut self, index: usize, input: I) -> Result<I, nom::Err<nom::error::Error<I>>>
//   where
//       D: Decoder<'a, I>,
//       Self: Sized,
// {{
//   let mut input = input;
//   match index {{
//     {decode_member_body}
//     _ => {extension_decoder}
//   }}
//   Ok(input)
// }}
// }}

// impl<'a, I: AsBytes + Debug + 'a> Decode<'a, I> for {name} {{
//   {DECODE_SIGNATURE}
//   {{
//     match {name}::decoder::<D>() {{
//       Ok(mut decoder) => decoder(input),
//       Err(_e) => Err(nom::Err::Error(nom::error::Error {{
//         input,
//         code: nom::error::ErrorKind::Fail,
//       }}))
//     }}
//   }}

//   {DECODER_SIGNATURE}
//   {{
//     D::decode_information_object({information_object_descriptor})
//   }}
// }}
// "#
//     )
// }

// pub fn information_object_set_template(
//     comments: String,
//     name: String,
//     options: String,
//     key_type: String,
//     for_key_branches: String,
// ) -> String {
//     format!(
//         r#"{comments}
// pub enum {name} {{
//   {options}
// }}

// impl<I: AsBytes + Debug> DecoderForKey<I, {key_type}> for {name} {{
//   fn decoder_for_key<I, D>(
//     key: {key_type},
//   ) -> Result<fn(I) -> IResult<I, Self>, DecodingError>
//   where
//     D: Decoder,
//     T: PartialEq,
//     Self: Sized,
//   {{
//     match key {{
//       {for_key_branches}
//     }}
//   }}
// }}
// "#
//     )
// }
