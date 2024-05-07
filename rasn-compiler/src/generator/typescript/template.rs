pub fn typealias_template(comments: &str, name: &str, alias: &str) -> String {
    format!(
        r#"{comments}
        export type {name} = {alias};"#
    )
}

pub fn value_template(comments: &str, name: &str, value: &str) -> String {
    format!(
        r#"{comments}
        export const {name} = {value};"#
    )
}

pub fn number_like_template(comments: &str, name: &str) -> String {
    format!(
        r#"{comments}
        export type {name} = number;"#
    )
}

pub fn string_like_template(comments: &str, name: &str) -> String {
    format!(
        r#"{comments}
        export type {name} = string;"#
    )
}

pub fn bit_string_template(comments: &str, name: &str, ty: &str) -> String {
    format!(
        r#"{comments}
        export type {name} = {ty};"#
    )
}

pub fn octet_string_template(comments: &str, name: &str) -> String {
    format!(
        r#"{comments}
        export type {name} = string | object;"#
    )
}

pub fn boolean_template(comments: &str, name: &str) -> String {
    format!(
        r#"{comments}
        export type {name} = boolean;"#
    )
}

pub fn null_template(comments: &str, name: &str) -> String {
    format!(
        r#"{comments}
        export type {name} = null;"#
    )
}

pub fn any_template(comments: &str, name: &str) -> String {
    format!(
        r#"{comments}
        export type {name} = any;"#
    )
}

pub fn enumerated_template(comments: &str, name: &str, enum_members: &str) -> String {
    format!(
        r#"{comments}
        export enum {name} {{
            {enum_members}
        }};"#
    )
}

#[allow(clippy::too_many_arguments)]
pub fn sequence_or_set_template(comments: &str, name: &str, members: &str) -> String {
    format!(
        r#"{comments}
        export type {name} = {members};"#
    )
}

pub fn sequence_or_set_of_template(comments: &str, name: &str, member_type: &str) -> String {
    format!(
        r#"{comments}
        export type {name} = {member_type}[];"#
    )
}

pub fn choice_template(comments: &str, name: &str, options: &str) -> String {
    format!(
        r#"{comments}
        export type {name} = {options};"#
    )
}
