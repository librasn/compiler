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
