pub fn main() {
    let secret_key = aead::SecretKey::default(); 
    file_encrypt(&secret_key).unwrap();
    println!("file made");
    let _ = file_decrypt("super.txt_encrypt.txt", &secret_key);
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

pub fn interface() -> bool {
    let mut input = String::new();          //stdin stuff

    println!("Insert Password: ");
    stdin().read_line(&mut input).unwrap();     //stdin stuff
    input.pop();                                    //gets rid of \n

    let password = pwhash::Password::from_slice(&input.as_bytes()).unwrap();    // sets password variable
    let hash = pwhash::hash_password(&password, 3, 1 << 16).unwrap();   //hash from password
    

    for _n in 1..3 {
        input.clear();                                      //gets rid of previous input
        println!("Enter Password:");        
        stdin().read_line(&mut input).unwrap();         //stdin stuff
        input.pop();
        if input.eq("-1") {                                 //exit on -1
            println!("Exiting password");
            return false;
        }
        match pwhash::hash_password_verify(&hash,&pwhash::Password::from_slice(input.as_bytes()).unwrap())  {   //compares set password to input
            Ok(_) => {
                println!("Password verified!");
                return true;
            },
            Err(_) => println!("Incorrect password."),
        }
    }
    false

}


pub fn write_to_file(s: &[u8], a: &str) ->  Result<File, std::io::Error> {
    let mut temp = format!("{}_encrypt.txt",a);
    let i = 0;
    while Path::new(&temp).exists() {
        temp = format!("{}{}",i.to_string(), temp);
    }
    let p = Path::new(&temp);
    let mut file = File::create(p)?;
    file.write_all(s)?;
    Ok(file)
}