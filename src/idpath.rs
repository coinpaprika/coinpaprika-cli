//! Validate and percent-encode a caller-supplied value so it can only ever
//! be sent as a single, inert URL path segment.
//!
//! Every CLI command that takes a coin, exchange, tag, person or platform ID
//! (or a contract address) builds a request path by interpolating that value
//! with `format!`. Without this step a value like ".." or "%2e%2e" can make
//! the final request go to a different path on the API host than the one
//! the command intended, carrying the user's Authorization header with it.
//!
//! Encoding alone is not enough: a percent-encoder leaves "." and ".."
//! untouched because they are in the RFC 3986 "unreserved" set, and a URL
//! parser can treat a percent-encoded ".." segment (for example "%2e%2e")
//! as a directory traversal before the request is even sent. So this
//! module validates first (on the value after one percent-decode pass) and
//! only then encodes the original value.

use anyhow::{bail, Result};
use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

/// RFC 3986 unreserved characters are left unescaped; everything else
/// (including '%' itself) is percent-encoded. This matches what
/// `encodeURIComponent`, `url.PathEscape` and `urllib.parse.quote(safe="")`
/// do for a single path segment.
const PATH_SEGMENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

/// Validate `id` and return it percent-encoded as exactly one URL path
/// segment.
///
/// Rejects:
/// - an empty value
/// - a value that is "." or ".."
/// - a value that, after one percent-decode pass, is not valid UTF-8
/// - a value that, after one percent-decode pass, is "." or ".."
/// - a value that, after one percent-decode pass, contains '/', '\\', '?',
///   '#', '%', whitespace or a control character
pub fn path_segment(id: &str) -> Result<String> {
    if id.is_empty() {
        bail!("id must not be empty");
    }

    let decoded_bytes: Vec<u8> = percent_decode_str(id).collect();
    let decoded = String::from_utf8(decoded_bytes)
        .map_err(|_| anyhow::anyhow!("id is not valid UTF-8 after percent-decoding"))?;

    if decoded == "." || decoded == ".." {
        bail!("id must not be \".\" or \"..\"");
    }

    if decoded.chars().any(|c| {
        c.is_control() || c.is_whitespace() || matches!(c, '/' | '\\' | '?' | '#' | '%')
    }) {
        bail!("id contains a character that is not allowed in a URL path segment");
    }

    Ok(utf8_percent_encode(id, PATH_SEGMENT).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty() {
        assert!(path_segment("").is_err());
    }

    #[test]
    fn rejects_literal_dot_segments() {
        assert!(path_segment(".").is_err());
        assert!(path_segment("..").is_err());
    }

    #[test]
    fn rejects_percent_encoded_dot_segments() {
        assert!(path_segment("%2e%2e").is_err());
        assert!(path_segment("%2E%2E").is_err());
        assert!(path_segment("%2e.").is_err());
        assert!(path_segment(".%2e").is_err());
        assert!(path_segment("%2e").is_err());
    }

    #[test]
    fn rejects_slash_backslash_query_fragment_percent() {
        assert!(path_segment("a/b").is_err());
        assert!(path_segment("a\\b").is_err());
        assert!(path_segment("x?y=1").is_err());
        assert!(path_segment("x#y").is_err());
        assert!(path_segment("..%2Fusage").is_err());
        assert!(path_segment("%2e%2e%2Fusage").is_err());
        assert!(path_segment("a%2fb").is_err());
        assert!(path_segment("50%").is_err());
    }

    #[test]
    fn rejects_whitespace_and_control_chars() {
        assert!(path_segment("a b").is_err());
        assert!(path_segment("a\tb").is_err());
        assert!(path_segment("a\nb").is_err());
        assert!(path_segment("a\u{1b}b").is_err());
    }

    #[test]
    fn allows_ordinary_ids_unchanged() {
        assert_eq!(path_segment("btc-bitcoin").unwrap(), "btc-bitcoin");
        assert_eq!(path_segment("eth-ethereum").unwrap(), "eth-ethereum");
        assert_eq!(path_segment("vitalik-buterin").unwrap(), "vitalik-buterin");
        assert_eq!(
            path_segment("0xdac17f958d2ee523a2206206994597c13d831ec7").unwrap(),
            "0xdac17f958d2ee523a2206206994597c13d831ec7"
        );
    }

    #[test]
    fn encodes_characters_outside_the_unreserved_set() {
        // A mid-string dot is fine (not a dot-segment) but anything that
        // would change how the URL is parsed still gets escaped.
        assert_eq!(path_segment("a.b").unwrap(), "a.b");
        assert_eq!(path_segment("a@b").unwrap(), "a%40b");
        assert_eq!(path_segment("a:b").unwrap(), "a%3Ab");
    }

    #[test]
    fn built_path_never_contains_a_dot_segment() {
        // Defense in depth: whatever passes validation, the encoded form
        // must never contain a bare "." or ".." component once placed
        // between slashes.
        for candidate in ["btc-bitcoin", "a.b", "..valid-looking..", "~tilde~"] {
            if let Ok(encoded) = path_segment(candidate) {
                for seg in encoded.split('/') {
                    assert_ne!(seg, "..");
                    assert_ne!(seg, ".");
                }
            }
        }
    }
}
