//! This module contains utility functions for string manipulation.

/// Converts a string to short Unicode representations.
///
/// This function takes a string and returns a new string where each character
/// is represented by the last two hexadecimal digits of its Unicode code point.
///
/// # Examples
///
/// ```
/// # use std::panic;
/// # fn main() -> Result<(), String> {
/// let result = string_to_short_unicode("A");
/// assert_eq!(result, "41");
///
/// let result = string_to_short_unicode("ABC");
/// assert_eq!(result, "414243");
///
/// let result = string_to_short_unicode("A中🚀");
/// assert_eq!(result, "412D80");
///
/// let result = string_to_short_unicode("");
/// assert_eq!(result, "");
///
/// let result = string_to_short_unicode(" ");
/// assert_eq!(result, "20");
///
/// let result = string_to_short_unicode("😊");
/// assert_eq!(result, "0A");
/// # Ok(())
/// # }
///
/// # fn string_to_short_unicode(input: &str) -> String {
/// #     input
/// #         .chars()
/// #         .map(|ch| format!("{:02X}", (ch as u32) & 0xFF))
/// #         .collect::<Vec<String>>()
/// #         .join("")
/// # }
/// ```
pub fn string_to_short_unicode(input: &str) -> String {
    input
        .chars()
        .map(|ch| format!("{:02X}", (ch as u32) & 0xFF))
        .collect::<Vec<String>>()
        .join("")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_short_unicode() {
        assert_eq!(string_to_short_unicode("A"), "41");
        assert_eq!(string_to_short_unicode("ABC"), "414243");
        assert_eq!(string_to_short_unicode("A中🚀"), "412D80");
        assert_eq!(string_to_short_unicode(""), "");
        assert_eq!(string_to_short_unicode(" "), "20");
        assert_eq!(string_to_short_unicode("😊"), "0A");
    }
}