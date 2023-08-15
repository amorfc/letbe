use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use serde::{Deserialize, Serialize};

use crate::config::ENV_CONFIG;

use super::datetime::LettDate;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LettJwtClaims {
    user_id: i32,
    device_id: String,
    exp: usize,
}

const DEFAULT_TOKEN_EXPIRE: usize = 60 * 60 * 24; // 1 days

impl LettJwtClaims {
    pub fn new(user_id: i32, device_id: String, exp: Option<usize>) -> Self {
        Self {
            user_id,
            device_id,
            exp: exp.unwrap_or_else(|| {
                LettDate::utc_now().timestamp_millis() as usize + DEFAULT_TOKEN_EXPIRE
            }),
        }
    }
}

#[derive(Debug)]
pub struct LettJwt {}

impl LettJwt {
    pub fn encode(claims: &LettJwtClaims) -> Result<String, String> {
        let encoded = encode::<LettJwtClaims>(&Self::header(), claims, &Self::encoding_key())
            .map_err(|e| e.to_string())?;

        Ok(encoded)
    }

    pub fn decode(jwt_token: &str) -> Result<LettJwtClaims, String> {
        let decoded =
            decode::<LettJwtClaims>(jwt_token, &Self::decoding_key(), &Self::validation())
                .map_err(|e| e.to_string())?;

        Ok(decoded.claims)
    }

    fn algorithm() -> Algorithm {
        Algorithm::HS256
    }

    fn validation() -> Validation {
        Validation::new(Self::algorithm())
    }

    fn encoding_key() -> EncodingKey {
        EncodingKey::from_secret(Self::secret().as_ref())
    }

    fn decoding_key() -> DecodingKey {
        DecodingKey::from_secret(Self::secret().as_ref())
    }

    fn header() -> Header {
        Header::new(Self::algorithm())
    }

    fn secret() -> String {
        ENV_CONFIG.jwt_secret.clone()
    }
}
