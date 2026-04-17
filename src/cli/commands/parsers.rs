//! Hex and base64 parsing utilities
//!
//! Provides safe parsing with validation for hex and base64 encoded data.

use crate::error::{Result, SerialError};

/// Parse a hex string into bytes with validation
///
/// # Examples
///
/// ```
/// parse_hex_string("01020304") => Ok([0x01, 0x02, 0x03, 0x04])
/// parse_hex_string("abc") => Err("Hex string must have even length")
/// parse_hex_string("XY") => Err("Non-hex character detected")
/// ```
pub fn parse_hex_string(s: &str) -> Result<Vec<u8>> {
    // Remove "0x" prefix if present
    let s = s.strip_prefix("0x").unwrap_or(s);

    // Validate: check for empty string
    if s.is_empty() {
        return Err(SerialError::InvalidInput("Hex string is empty".to_string()));
    }

    // Validate: check for non-hex characters
    if !s.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(SerialError::InvalidInput(
            "Non-hex character detected in hex string".to_string(),
        ));
    }

    // Validate: check for even length
    if s.len() % 2 != 0 {
        return Err(SerialError::InvalidInput(
            "Hex string must have even length".to_string(),
        ));
    }

    // Parse the validated string
    (0..s.len())
        .step_by(2)
        .map(|i| {
            let byte_chars: Vec<char> = s[i..i + 2].chars().collect();
            let byte_str: String = byte_chars.iter().collect();
            u8::from_str_radix(&byte_str, 16).map_err(|_| {
                SerialError::InvalidInput(format!(
                    "Invalid hex byte at position {}: '{}{}'",
                    i, byte_chars[0], byte_chars[1]
                ))
            })
        })
        .collect()
}

/// Decode a base64 string with validation
///
/// # Examples
///
/// ```
/// base64_decode("SGVsbG8=") => Ok("Hello".as_bytes())
/// base64_decode("invalid!") => Err("Invalid base64")
/// ```
pub fn base64_decode(s: &str) -> Result<Vec<u8>> {
    use base64::Engine as _;

    // Validate: check for empty string
    if s.is_empty() {
        return Err(SerialError::InvalidInput("Base64 string is empty".to_string()));
    }

    // Check for common base64 characters
    if !s.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=' || c == '\n' || c == '\r'
    }) {
        return Err(SerialError::InvalidInput(
            "Invalid character in base64 string".to_string(),
        ));
    }

    // Remove newlines if present
    let s = s.replace('\n', "").replace('\r', "");

    // Decode with detailed error message
    base64::engine::general_purpose::STANDARD
        .decode(&s)
        .map_err(|e| SerialError::InvalidInput(format!("Invalid base64: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_valid() {
        assert_eq!(parse_hex_string("01020304").unwrap(), vec![0x01, 0x02, 0x03, 0x04]);
        assert_eq!(parse_hex_string("0x01020304").unwrap(), vec![0x01, 0x02, 0x03, 0x04]);
        assert_eq!(parse_hex_string("FF").unwrap(), vec![0xFF]);
        assert_eq!(parse_hex_string("00").unwrap(), vec![0x00]);
    }

    #[test]
    fn test_parse_hex_invalid() {
        assert!(parse_hex_string("").is_err()); // Empty
        assert!(parse_hex_string("abc").is_err()); // Odd length
        assert!(parse_hex_string("XYZ").is_err()); // Non-hex chars
        assert!(parse_hex_string("12 34").is_err()); // Space
    }

    #[test]
    fn test_base64_decode_valid() {
        assert_eq!(base64_decode("SGVsbG8=").unwrap(), b"Hello");
        assert_eq!(base64_decode("AAEC").unwrap(), vec![0x00, 0x01, 0x02]);
    }

    #[test]
    fn test_base64_decode_invalid() {
        assert!(base64_decode("").is_err()); // Empty
        assert!(base64_decode("invalid!@#").is_err()); // Invalid chars
    }
}
