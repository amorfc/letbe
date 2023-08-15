use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
};

#[derive(Debug, Clone)]
pub struct LettHasher {
    pub hashed: String,
    pub salt: String,
}

impl LettHasher {
    fn argon2_hasher() -> Argon2<'static> {
        let params = Params::new(
            4096, // Amunt of memory to use (memory cost)
            2,    // Number of iterations (time cost)
            1, None,
        )
        .unwrap_or_default();

        Argon2::new(
            argon2::Algorithm::Argon2id, // Algorithm variant
            argon2::Version::V0x10,
            params,
        )
    }
    pub fn hash_with_salt(password: &str) -> Result<Self, String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = LettHasher::argon2_hasher()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| e.to_string())?;

        Ok(Self {
            hashed: hash.to_string(),
            salt: salt.to_string(),
        })
    }

    pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(password_hash).map_err(|e| e.to_string())?;

        Ok(LettHasher::argon2_hasher()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
