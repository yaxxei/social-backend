pub mod error;

use argon2::Config;
use rand_core::{OsRng, RngCore};

pub fn hash_password(password: &str) -> error::Result<String> {
    let config = Config::default();
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    argon2::hash_encoded(password.as_bytes(), &salt, &config).map_err(|_| error::Error::Hash)
}

pub fn validate_password(password: &str, hashed: &str) -> error::Result<bool> {
    argon2::verify_encoded(hashed, password.as_bytes()).map_err(|_| error::Error::Validate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};

    #[test]
    fn test_hash_password_success() -> Result<()> {
        let password = "secure_password";

        let hashed_password = hash_password(password).context("Failed to hash password")?;

        assert!(
            !hashed_password.is_empty(),
            "Hashed password should not be empty"
        );

        Ok(())
    }

    #[test]
    fn test_validate_password_success() -> Result<()> {
        let password = "secure_password";

        let hashed_password = hash_password(password).context("Failed to hash password")?;

        let is_valid =
            validate_password(password, &hashed_password).context("Failed to validate password")?;

        assert!(is_valid, "Password should match the hash");

        Ok(())
    }

    #[test]
    fn test_validate_password_failure() -> Result<()> {
        let password = "secure_password";

        let hashed_password = hash_password(password).context("Failed to hash password")?;

        let wrong_password = "wrong_password";

        let is_valid = validate_password(wrong_password, &hashed_password)
            .context("Failed to validate password with wrong input")?;

        assert!(!is_valid, "Wrong password should not match the hash");

        Ok(())
    }

    #[test]
    fn test_validate_password_invalid_hash() -> Result<()> {
        let password = "secure_password";
        let invalid_hash = "invalid_hash";

        let result = validate_password(password, invalid_hash);

        assert!(
            result.is_err(),
            "Validation should fail for an invalid hash"
        );

        Ok(())
    }
}
