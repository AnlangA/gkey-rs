use rand::Rng;
use rand::rngs::OsRng;
use rand::distributions::Uniform;

/// Defines the type of characters to include in the generated password.
#[derive(PartialEq)]
pub enum PasswordType {
    Alphanumeric,   // Only letters and digits
    Alphabetic,     // Only letters
    Numeric,        // Only digits
    SpecialChars,   // Only special characters
    All,            // All characters
}
pub const AlPHANUMERIC: &str = r#"大写字母、小写字母、数字"#;
pub const AIPHABETIC: &str = r#"大写字母、小写字母"#;
pub const NUMERIC: &str = r#"数字"#;
pub const SPECIALCHARS: &str = r#"特殊符号"#;
pub const ALL: &str = r#"大小写字母、数字、特殊符号"#;
/// Generates a random password of a specified length and type.
///
/// This function uses the `rand` crate to generate a password consisting of
/// different types of characters based on the specified `PasswordType`. The
/// randomness is sourced from the operating system's entropy pool via `OsRng`.
///
/// # Arguments
///
/// * `length` - The length of the password to generate.
/// * `password_type` - The type of characters to include in the password.
///
/// # Returns
///
/// A `String` containing the generated random password.
///
/// # Examples
///
/// ```
/// use your_crate::{generate_random_password, PasswordType};
///
/// let password = generate_random_password(12, PasswordType::All);
/// assert_eq!(password.len(), 12);
/// println!("Generated password: {}", password);
/// ```
///
/// # Panics
///
/// This function will panic if it fails to generate randomness from the operating system.
pub fn generate_random_password(length: usize, password_type: PasswordType) -> String {
    const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const DIGITS: &[u8] = b"0123456789";
    const SPECIAL: &[u8] = b"!@#$%^&*()";

    let charset: Vec<u8> = match password_type {
        PasswordType::Alphanumeric => [UPPERCASE, LOWERCASE, DIGITS].concat(),
        PasswordType::Alphabetic => [UPPERCASE, LOWERCASE].concat(),
        PasswordType::Numeric => DIGITS.to_vec(),
        PasswordType::SpecialChars => SPECIAL.to_vec(),
        PasswordType::All => [UPPERCASE, LOWERCASE, DIGITS, SPECIAL].concat(),
    };

    let char_range = Uniform::from(0..charset.len());
    let mut rng = OsRng;

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.sample(&char_range);
            charset[idx] as char
        })
        .collect();

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_password_length() {
        let length = 12;
        let password = generate_random_password(length, PasswordType::All);
        assert_eq!(password.len(), length);
    }

    #[test]
    fn test_generate_random_password_non_empty() {
        let password = generate_random_password(1, PasswordType::All);
        assert!(!password.is_empty());
    }

    #[test]
    fn test_generate_random_password_various_lengths() {
        for length in 1..=100 {
            let password = generate_random_password(length, PasswordType::All);
            assert_eq!(password.len(), length);
        }
    }

    #[test]
    fn test_generate_random_password_alphanumeric() {
        let password = generate_random_password(12, PasswordType::Alphanumeric);
        assert!(password.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_generate_random_password_alphabetic() {
        let password = generate_random_password(12, PasswordType::Alphabetic);
        assert!(password.chars().all(|c| c.is_ascii_alphabetic()));
    }

    #[test]
    fn test_generate_random_password_numeric() {
        let password = generate_random_password(12, PasswordType::Numeric);
        assert!(password.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_random_password_special_chars() {
        let password = generate_random_password(12, PasswordType::SpecialChars);
        assert!(password.chars().all(|c| "!@#$%^&*()".contains(c)));
    }
}
