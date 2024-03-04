use orion::aead;
use orion::pwhash::{self, PasswordHash};
use orion::errors::UnknownCryptoError;

fn main() {
    println!("Hello, world!");
    let _ = encrypt();
    let _ = psswd();
}
pub fn encrypt() -> Result<(), UnknownCryptoError>{
    let secret_key = aead::SecretKey::default();
    let ciphertext = aead::seal(&secret_key, b"Secret message")?;
    let decrypted_data = aead::open(&secret_key, &ciphertext)?;
    let text = match String::from_utf8(ciphertext) {
        Ok(s) => s,
        Err(_) => String::from("Error"), 
    };
    println!("Cipher Text: {:?}", text);
    println!("Decpyhered Text: {:?}", decrypted_data);
    println!("Decpyhered Text: {:?}", String::from_utf8(decrypted_data).expect("sad"));

    Ok(())
}
pub fn psswd() -> Result<(), UnknownCryptoError> {
    let password = pwhash::Password::from_slice(b"Secret password")?;
    let second = pwhash::Password::from_slice(b"secret password")?;
    let hash = pwhash::hash_password(&password, 3, 1<<16)?;
    assert!(pwhash::hash_password_verify(&hash, &password).is_ok());

    println!("Password: {}", PasswordHash::unprotected_as_encoded(&hash));
    Ok(())
}