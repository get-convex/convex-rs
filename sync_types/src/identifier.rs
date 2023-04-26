use anyhow::bail;

pub const MAX_IDENTIFIER_LEN: usize = 64;

pub const IDENTIFIER_REQUIREMENTS: &str =
    "Identifiers must start with a letter and can only contain letters, digits, and underscores.";

/// Check that a string to be used for a table name or field name.
///
/// We use a simplified ASCII version of Rust's syntax [^1] which also overlaps
/// with JavaScript's syntax [^2].
///
/// ```text
/// ident: start continue*
/// start: a-zA-z_
/// continue: a-zA-Z0-9_
/// ```
/// [^3][^4]
/// To be conservative, let's also ban identifiers of entirely `_` too.
///
/// [1]: <https://doc.rust-lang.org/reference/identifiers.html>
/// [2]: <https://developer.mozilla.org/en-US/docs/Glossary/Identifier>
/// [3]: <https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%3AXID_START%3A%5D&g=&i=>
/// [4]: <https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%3AXID_CONTINUE%3A%5D&g=&i=>
pub fn check_valid_identifier(s: &str) -> anyhow::Result<()> {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() => (),
        Some('_') => (),
        Some(c) => bail!(
            "Invalid first character '{c}' in {s}: Identifiers must start with an alphabetic \
             character or underscore"
        ),
        None => bail!("Identifier cannot be empty"),
    };
    for c in chars {
        if !c.is_ascii_alphanumeric() && c != '_' {
            bail!(
                "Identifier {s} has invalid character '{c}': Identifiers can only contain \
                 alphanumeric characters or underscores"
            );
        }
    }
    if s.len() > MAX_IDENTIFIER_LEN {
        bail!(
            "Identifier is too long ({} > maximum {})",
            s.len(),
            MAX_IDENTIFIER_LEN
        );
    }
    if s.chars().all(|c| c == '_') {
        bail!("Identifier {s} cannot have exclusively underscores");
    }
    Ok(())
}

pub const MIN_IDENTIFIER: &str = "A";

#[cfg(any(test, feature = "testing"))]
pub mod arbitrary_regexes {
    pub const IDENTIFIER_REGEX: &str = "[a-zA-Z_][a-zA-Z][a-zA-Z0-9_]{0,62}";
    pub const USER_IDENTIFIER_REGEX: &str = "[a-zA-Z][a-zA-Z0-9_]{0,63}";
    pub const SYSTEM_IDENTIFIER_REGEX: &str = "_[a-zA-Z][a-zA-Z0-9_]{0,62}";
    pub const ENV_VAR_NAME_REGEX: &str = "_[a-zA-Z][a-zA-Z0-9_]{0,38}";
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::{
        arbitrary_regexes::IDENTIFIER_REGEX,
        MIN_IDENTIFIER,
    };

    proptest! {
        #![proptest_config(ProptestConfig { failure_persistence: None, .. ProptestConfig::default() })]

        #[test]
        fn test_min_identifier(ident in IDENTIFIER_REGEX) {
            assert!(MIN_IDENTIFIER <= &ident[..]);
        }
    }
}
