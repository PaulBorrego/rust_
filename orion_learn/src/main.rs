use orion::aead;
use orion::pwhash::{self, PasswordHash};
use orion::errors::UnknownCryptoError;
use std::io::stdin;
fn main() {
    println!("Hello, world!");
    // let _ = encrypt();
    // let _ = psswd();

    // let password = pwhash::Password::from_slice(b"password").unwrap();
    // let hash = pwhash::hash_password(&password, 3, 1 << 16).unwrap();

    // let t = pwhash::Password::from_slice(b"password").unwrap();
    // let f = pwhash::Password::from_slice(b"notword").unwrap();

    // match pwhash::hash_password_verify(&hash, &t)  {
    //     Ok(_) => println!("Password verified!"),
    //     Err(_) => println!("Incorrect password."),
    // }
    // match pwhash::hash_password_verify(&hash, &f) {
    //     Ok(_) => println!("Bad match"),
    //     Err(_) => println!("Good Fail"),
    // }
    interface();
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
    // let second = pwhash::Password::from_slice(b"P@uljb108!1804")?;
    let hash = pwhash::hash_password(&password, 3, 1<<16)?;
    assert!(pwhash::hash_password_verify(&hash, &password).is_ok());

    println!("Password: {}", PasswordHash::unprotected_as_encoded(&hash));
    // println!("P@uljb108!1804: {}" PasswordHash::unprotected_as_encoded())
    Ok(())
}

pub fn interface() -> () {
    let mut input = String::new();          //stdin stuff
    let mut l = true; 

    println!("Insert Password: ");
    stdin().read_line(&mut input).unwrap();     //stdin stuff
    input.pop();                                    //gets rid of \n

    let password = pwhash::Password::from_slice(&input.as_bytes()).unwrap();    // sets password variable
    let hash = pwhash::hash_password(&password, 3, 1 << 16).unwrap();   //hash from password
    
    while l {
        input.clear();                                      //gets rid of previous input
        println!("Enter Password:");        
        stdin().read_line(&mut input).unwrap();         //stdin stuff
        input.pop();
        if input.eq("-1") {                                 //exit on -1
            println!("Exiting password");
            l = false;
        }
        match pwhash::hash_password_verify(&hash,&pwhash::Password::from_slice(input.as_bytes()).unwrap())  {   //compares set password to input
            Ok(_) => {
                print!("Password verified!");
                l = false;
            },
            Err(_) => println!("Incorrect password."),
        }

    }

}