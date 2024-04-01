use dotenv::dotenv;
use std::num::NonZeroU32;

use data_encoding::HEXUPPER;
use ring::pbkdf2;

pub fn hash_password(password: &String) -> String {
    dotenv().ok();

    let n_iter = NonZeroU32::new(100_000).unwrap();
    let mut pbkdf2_hash = [0u8; 64];
    let salt = dotenv::var("SALT").unwrap();

    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt.as_bytes(),
        password.as_bytes(),
        &mut pbkdf2_hash,
    );

    let hash = HEXUPPER.encode(&pbkdf2_hash);

    hash
}

pub fn verify_password(password: &String, verify_hash: &String) -> bool {
    let new_hash = hash_password(&password);

    &new_hash == verify_hash
}
