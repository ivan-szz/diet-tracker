use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::Algorithm::Argon2id;
use argon2::Version::V0x13;
use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use std::env;
use std::sync::OnceLock;

static HASH_SECRET: OnceLock<String> = OnceLock::new();

fn get_hash_secret() -> &'static str {
    HASH_SECRET.get_or_init(|| env::var("HASH_SECRET").expect("Missing HASH_SECRET"))
}

static ARGON2: OnceLock<Argon2<'static>> = OnceLock::new();

fn get_argon2() -> &'static Argon2<'static> {
    ARGON2.get_or_init(|| {
        let secret = get_hash_secret();
        Argon2::new_with_secret(secret.as_bytes(), Argon2id, V0x13, Params::default())
            .expect("Unable to create argon2 instance")
    })
}

pub fn hash(string: &str) -> Result<String, argon2::password_hash::Error> {
    let argon2 = get_argon2();
    let salt = SaltString::generate(&mut OsRng);
    let hashed_string = argon2.hash_password(string.as_bytes(), &salt)?.to_string();
    Ok(hashed_string)
}

pub fn verify(string: &str, hash: &str) -> Result<(), argon2::password_hash::Error> {
    let argon2 = get_argon2();
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(argon2.verify_password(string.as_bytes(), &parsed_hash)?)
}
