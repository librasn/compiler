use std::str::FromStr;

pub fn int_type_token<'a>(min: i128, max: i128, is_extensible: bool) -> &'a str {
    if is_extensible {
        "Integer"
    } else if min >= 0 {
        match max {
            r if r <= u8::MAX.into() => "u8",
            r if r <= u16::MAX.into() => "u16",
            r if r <= u32::MAX.into() => "u32",
            r if r <= u64::MAX.into() => "u64",
            _ => "Integer",
        }
    } else {
        match (min, max) {
            (mi, ma) if mi >= i8::MIN.into() && ma <= i8::MAX.into() => "i8",
            (mi, ma) if mi >= i16::MIN.into() && ma <= i16::MAX.into() => "i16",
            (mi, ma) if mi >= i32::MIN.into() && ma <= i32::MAX.into() => "i32",
            (mi, ma) if mi >= i64::MIN.into() && ma <= i64::MAX.into() => "i64",
            _ => "Integer",
        }
    }
}

const RUST_KEYWORDS: [&'static str; 38] = [
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

pub fn to_rust_snake_case(input: &String) -> Ident {
    let mut input = input.replace("-", "_");
    let input = input.drain(..).fold(String::new(), |mut acc, c| {
        if acc.is_empty() && c.is_uppercase() {
            acc.push(c.to_ascii_lowercase());
        } else if acc.ends_with(|last: char| last.is_lowercase() || last == '_') && c.is_uppercase()
        {
            acc.push('_');
            acc.push(c.to_ascii_lowercase());
        } else {
            acc.push(c);
        }
        acc
    });
    let name = if RUST_KEYWORDS.contains(&input.as_str()) {
        String::from("r_") + &input
    } else {
        input
    };
    Ident::new(&name, Span::call_site())
}

pub fn to_rust_const_case(input: &String) -> Ident {
    Ident::new(
        &to_rust_snake_case(input).to_string().to_uppercase(),
        Span::call_site(),
    )
}

pub fn to_rust_enum_identifier(input: &String) -> Ident {
    let mut formatted = format_ident!("{}", input.replace("-", "_"));
    if RUST_KEYWORDS.contains(&input.as_str()) {
        formatted = format_ident!("R_{formatted}");
    }
    formatted
}

pub fn to_rust_title_case(input: &String) -> TokenStream {
    let mut input = input.replace("-", "_");
    let input = input.drain(..).fold(String::new(), |mut acc, c| {
        if acc.is_empty() && c.is_lowercase() {
            acc.push(c.to_ascii_uppercase());
        } else if acc.ends_with(|last: char| last == '_') && c.is_uppercase() {
            acc.pop();
            acc.push(c);
        } else if acc.ends_with(|last: char| last == '_') {
            acc.pop();
            acc.push(c.to_ascii_uppercase());
        } else {
            acc.push(c);
        }
        acc
    });
    let name = if RUST_KEYWORDS.contains(&input.as_str()) {
        String::from("R_") + &input
    } else {
        input
    };
    TokenStream::from_str(&name).unwrap()
}

macro_rules! get_declaration {
    ($tlds:ident, $key:expr, $tld_ty:ident, $asn1_ty:path) => {{
        if let Some(tld) = $tlds.get($key) {
            match tld {
                ToplevelDefinition::$tld_ty(inner) => match inner.pdu() {
                    $asn1_ty(asn) => Some(asn),
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        }
    }};
}

pub(crate) use get_declaration;
use proc_macro2::{Ident, Span, TokenStream};
use quote::format_ident;

#[cfg(test)]
mod tests {
    use super::int_type_token;

    #[test]
    fn determines_int_type() {
        assert_eq!(int_type_token(600, 600, false), "u16");
        assert_eq!(int_type_token(0, 0, false), "u8");
        assert_eq!(int_type_token(-1, 1, false), "i8");
        assert_eq!(
            int_type_token(0, 124213412341389457931857915125, false),
            "Integer"
        );
        assert_eq!(int_type_token(-67463, 23123, false), "i32");
        assert_eq!(int_type_token(255, 257, false), "u16");
    }
}
