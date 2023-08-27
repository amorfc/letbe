use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use serde::{Deserialize, Serialize};

use crate::config::ENV_CONFIG;

use super::datetime::LettDate;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LettJwtClaims {
    pub user_id: i32,
    pub device_id: String,
    pub exp: usize,
}

const DEF_ACCESS_TOKEN_EXPIRE_MS: usize = 60 * 60 * 1000; // 1 hour
const DEF_REFRESH_TOKEN_EXPIRE_MS: usize = 60 * 60 * 24 * 7 * 1000; // 1 week

impl LettJwtClaims {
    pub fn access_token(user_id: i32, device_id: String, exp: Option<usize>) -> Self {
        Self {
            user_id,
            device_id,
            exp: exp.unwrap_or_else(|| {
                LettDate::utc_now().timestamp_millis() as usize + DEF_ACCESS_TOKEN_EXPIRE_MS
            }),
        }
    }

    pub fn refresh_token(user_id: i32, device_id: String, exp: Option<usize>) -> Self {
        Self {
            user_id,
            device_id,
            exp: exp.unwrap_or_else(|| {
                LettDate::utc_now().timestamp_millis() as usize + DEF_REFRESH_TOKEN_EXPIRE_MS
            }),
        }
    }
}

#[derive(Debug)]
pub struct LettJwt {}

impl LettJwt {
    pub fn create_jwt(claims: &LettJwtClaims) -> Result<String> {
        Self::encode(claims)
    }
    pub fn expose_jwt(jwt_token: &str) -> Result<LettJwtClaims> {
        Self::decode(jwt_token)
    }

    fn encode(claims: &LettJwtClaims) -> Result<String> {
        let encoded = encode::<LettJwtClaims>(&Self::header(), claims, &Self::encoding_key())?;

        Ok(encoded)
    }

    fn decode(jwt_token: &str) -> Result<LettJwtClaims> {
        let decoded =
            decode::<LettJwtClaims>(jwt_token, &Self::decoding_key(), &Self::validation())?;

        Ok(decoded.claims)
    }

    pub fn refresh_access_refresh_tokens(
        refresh_token: &str,
        access_expr: Option<usize>,
        refresh_expr: Option<usize>,
    ) -> Result<(String, String)> {
        let decoded_claims = Self::decode(refresh_token)?;

        let access_token_claims = LettJwtClaims::access_token(
            decoded_claims.user_id,
            decoded_claims.device_id.clone(),
            access_expr,
        );

        let refresh_token_claims = LettJwtClaims::refresh_token(
            decoded_claims.user_id,
            decoded_claims.device_id,
            refresh_expr,
        );

        let access_token = Self::encode(&access_token_claims)?;
        let refresh_token = Self::encode(&refresh_token_claims)?;

        Ok((access_token, refresh_token))
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

    fn get_access_token_user_claims(expr: Option<usize>) -> LettJwtClaims {
        let user_id = 31;
        let device_id = "AppleMacbook123123".to_string();

        LettJwtClaims::access_token(user_id, device_id, expr)
    }

    fn get_refresh_token_user_claims(expr: Option<usize>) -> LettJwtClaims {
        let user_id = 31;
        let device_id = "AppleMacbook123123".to_string();

        LettJwtClaims::refresh_token(user_id, device_id, expr)
    }

    #[test]

    fn valid_jwt_tests() {
        load_dummy_envs();
        let claims = get_access_token_user_claims(None);

        let token = LettJwt::create_jwt(&claims).unwrap();

        let decoded_claims = LettJwt::expose_jwt(&token).unwrap();

        assert_eq!(claims, decoded_claims);
    }

    #[test]
    fn invalid_jwt_tests() {
        load_dummy_envs();
        let mut claims = get_access_token_user_claims(None);

        let token = LettJwt::create_jwt(&claims).unwrap();

        claims.user_id = 0;

        let decoded_claims = LettJwt::decode(&token).unwrap();

        assert_ne!(claims, decoded_claims);
    }

    #[test]
    fn expired_jwt_tests() {
        load_dummy_envs();

        let one_sec_dur = Duration::from_secs(1);
        let expr_millis = one_sec_dur.as_millis() as usize;
        let claims = get_access_token_user_claims(Some(expr_millis));

        let token = LettJwt::create_jwt(&claims).unwrap();

        thread::sleep(one_sec_dur);

        let rs = LettJwt::expose_jwt(&token);
        let is_error = rs.is_err();
        let error_message = rs.unwrap_err().to_string();

        assert!(is_error);
        assert_eq!(error_message, "ExpiredSignature".to_string());
    }

    #[test]
    fn test_refresh_token() {
        load_dummy_envs();

        let access_token_claims = get_access_token_user_claims(None);
        let refresh_token_claims = get_refresh_token_user_claims(None);

        let access_token = LettJwt::create_jwt(&access_token_claims).unwrap();
        let refresh_token = LettJwt::create_jwt(&refresh_token_claims).unwrap();

        let decoded_access_token_claims = LettJwt::expose_jwt(&access_token).unwrap();
        let decoded_refresh_token_claims = LettJwt::expose_jwt(&refresh_token).unwrap();

        assert_eq!(
            decoded_access_token_claims.device_id,
            decoded_refresh_token_claims.device_id
        );
        assert_eq!(
            decoded_access_token_claims.user_id,
            decoded_refresh_token_claims.user_id
        );

        let (new_access_token, new_refresh_token) =
            LettJwt::refresh_access_refresh_tokens(&refresh_token, None, None).unwrap();

        let new_decoded_access_token_claims = LettJwt::expose_jwt(&new_access_token).unwrap();
        let new_decoded_refresh_token_claims = LettJwt::expose_jwt(&new_refresh_token).unwrap();

        assert_eq!(
            decoded_access_token_claims.device_id,
            new_decoded_access_token_claims.device_id
        );

        assert_eq!(
            decoded_access_token_claims.user_id,
            new_decoded_access_token_claims.user_id
        );

        assert_eq!(
            decoded_refresh_token_claims.user_id,
            new_decoded_refresh_token_claims.user_id
        );

        assert_eq!(
            decoded_refresh_token_claims.device_id,
            new_decoded_refresh_token_claims.device_id
        );
        dbg!(&new_decoded_refresh_token_claims);
        dbg!(&new_decoded_access_token_claims);
        assert!(new_decoded_refresh_token_claims.exp > new_decoded_access_token_claims.exp);
        assert!(new_decoded_refresh_token_claims.exp != new_decoded_access_token_claims.exp);
    }
}
