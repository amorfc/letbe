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

#[cfg(test)]
mod tests {
    use std::{env, thread, time::Duration};

    use super::*;
    fn load_dummy_envs() {
        env::set_var("DB", "test_db");
        env::set_var("DB_PORT", "5432");
        env::set_var("HOST", "localhost");
        env::set_var("HOST_PORT", "8080");
        env::set_var("POSTGRES_USER", "test_user");
        env::set_var("POSTGRES_PASSWORD", "test_password");
        env::set_var("SECRET", "test_secret");
        env::set_var("ENVIRONMENT", "test");
    }

    #[test]

    fn valid_jwt_tests() {
        load_dummy_envs();
        let user_id = 31;
        let device_id = "AppleMacbook123123".to_string();
        let claims = LettJwtClaims::new(user_id, device_id, None);

        let token = LettJwt::encode(&claims).unwrap();

        let decoded_claims = LettJwt::decode(&token).unwrap();

        assert_eq!(claims, decoded_claims);
    }

    #[test]
    fn invalid_jwt_tests() {
        load_dummy_envs();
        let user_id = 31;
        let device_id = "AppleMacbook123123".to_string();
        let mut claims = LettJwtClaims::new(user_id, device_id, None);

        let token = LettJwt::encode(&claims).unwrap();

        claims.user_id = 0;

        let decoded_claims = LettJwt::decode(&token).unwrap();

        assert_ne!(claims, decoded_claims);
    }

    #[test]
    fn expired_jwt_tests() {
        load_dummy_envs();

        let one_sec_dur = Duration::from_secs(1);
        let expr_millis = one_sec_dur.as_millis() as usize;
        let user_id = 31;
        let device_id = "AppleMacbook123123".to_string();
        let claims = LettJwtClaims::new(user_id, device_id, Some(expr_millis));

        let token = LettJwt::encode(&claims).unwrap();

        thread::sleep(one_sec_dur);

        let rs = LettJwt::decode(&token);
        let is_error = rs.is_err();
        let error_message = rs.unwrap_err();

        assert!(is_error);
        assert_eq!(error_message, "ExpiredSignature".to_string());
    }
}
