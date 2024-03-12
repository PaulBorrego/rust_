use orion::aead;
use orion::pwhash::{self, PasswordHash};
use orion::errors::UnknownCryptoError;
use std::error::Error;
// use std::fs;
use std::fs::{self, File};
use std::io::{stdin, Write};
// use std::io;
use std::path::Path;


fn main() {
    // println!("Hello, world!");
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
    // interface();
    encrypt_interface();

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
    let _ = add_encrypt(hash.unprotected_as_encoded());
    

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
pub fn add_encrypt (s: &str) -> std::io::Result<()> {
    let mut file = File::create("hold.txt")?;
    file.write_all(s.as_bytes())?;
    Ok(())
}
pub fn encrypt_data(s: &[u8], a: String) ->  Result<File, std::io::Error> {
    let temp = format!("{}_encrypt.txt",a);
    println!("{temp}");
    let p = Path::new(&temp);
    let mut file = File::create(p)?;
    file.write_all(s)?;
    Ok(file)
}

pub fn encrypt_interface() -> () {
    //todo ask for user id
    //todo ask for password

    let mut input = String::new();
    println!("What would you like to encrypt? (s)tring or (f)ile");
    stdin().read_line(&mut input).unwrap();
    
    let encrypted_file = match input.chars().next().unwrap() {
        's' => string_encrypt().unwrap(),
        'f' => file_encrypt().unwrap(),
        _ => panic!("AHHH"), 
    };
    
}

pub fn string_encrypt() -> Result<File, Box<dyn Error>>{
    println!("Type string you'd like to be encrypted: ");
    let mut buf = String::new();
    stdin().read_line(& mut buf).unwrap();
    let secret_key = aead::SecretKey::default();

    let text = aead::seal(&secret_key, buf.as_bytes())?; 
    match encrypt_data(&text, String::from("string")) {
        Ok(f) => Ok(f),
        Err(_) => Err("Writing Error")?,
    }
}
pub fn file_encrypt() -> Result<File, Box<dyn Error>>{
    println!("Type file path you'd like to be encrypted: ");
    let mut buf = String::new();
    stdin().read_line(& mut buf).unwrap();
    buf.pop();    //gets rid of \n
    let secret_key = aead::SecretKey::default();

    let contents = fs::read_to_string(buf.clone())?;
    println!("{}", contents);
    // println!("{:?}",String::from_utf8(contents.clone()));
    let text = aead::seal(&secret_key, &contents.as_bytes())?; 

    match encrypt_data(&text,buf.to_string()) {
        Ok(f) => Ok(f),
        Err(_) => Err("Writing Error")?
    }
}

