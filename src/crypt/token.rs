use crate::config;
use crate::crypt::{Error, Result};
use std::fmt::{Display, Formatter};

// region:    --- Token Type

/// String format: `ident_b64u.exp_b64u.sign_b64u`
pub struct Token {
    pub ident: String,     // Identifier (username for example)
    pub exp: String,       // Expiration date in RFC3339
    pub sign_b64u: String, // Signature, base64url encoded
}

// FIXME: FromStr

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// endregion: --- Token Type

// region:    --- Web Token generation & validation

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = config();
    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> Result<()> {
    let config = config();
    _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)?;
    Ok(())
}

// endregion: --- Web Token generation & validation

// region:    ---(private) token generation & validation

fn _generate_token(indent: &str, duration_sec: f64, salt: &str, key: &[u8]) -> Result<Token> {
    todo!()
}

fn _validate_token_sign_and_exp(origin_token: &Token, salt: &str, key: &[u8]) -> Result<()> {
    todo!()
}

/// Create token signature from token parts and salt
fn _token_sign_into_b64u(indent: &str, exp: &str, salt: &str, key: &[u8]) -> Result<String> {
    todo!()
}

// endregion: ---(private) token generation & validation

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_web_token() {
        todo!()
    }
}
// endregion: --- Tests
