use orion::aead;
use orion::hazardous::aead::streaming::SecretKey;
use orion::pwhash::{self};//, PasswordHash};
// use orion::errors::UnknownCryptoError;
use std::error::Error;
// use std::fs;
use std::fs::{self, File};
use std::io::{stdin, Write};
// use std::io;
use std::path::Path;



fn main() {    
    encrypt_interface();
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


pub fn encrypt_interface() -> () {
    let mut input = String::new();
    // let mut user_map = HashMap::new();

    // let v = fs::read_to_string("~/keys.tsv").unwrap().lines();
    // for s in v {
    //     user_map.insert(s.split_whitespace().next().unwrap(),User::from_tsv(s));
    // }
    
    // println!("(r)eturn user or (n)ew");
    // stdin().read_line(&mut input).unwrap();
    // let u = match input.chars().next().unwrap() {
    //     'r' => {
    //         println!("Type username");
    //         input.clear();
    //         stdin().read_line(&mut input).unwrap();
    //         input.pop();
    //         *user_map.get(&input.as_str()).unwrap()
    //     }
    //     'n' => {
    //         // let n = User::new();
    //         // let i = n.username.clone();
    //         // fs::write("~/keys.txt", User::to_tsv(&n)).expect("Failed to add tsv");
    //         // user_map.insert(i.as_str(), n);
    //         // user_map.get(&i.as_str()).unwrap()
    //         User::new()
    //     }
    //     _ => panic!("not r or n"),
    // };

    if interface() {
        let secret_key = aead::SecretKey::default();
        loop {
            input.clear();
            println!("Would you like to (e)ncrypt or (d)ecrypt");
            stdin().read_line(&mut input).unwrap();

            match input.chars().next().unwrap() {
                'e' => {
                    input.clear();
                    println!("What would you like to encrypt? (s)tring or (f)ile");
                    stdin().read_line(&mut input).unwrap();
                    
                     match input.chars().next().unwrap() {
                        's' => string_encrypt(&secret_key).unwrap(),
                        'f' => file_encrypt(&secret_key).unwrap(),
                        _ => panic!("not s or f"), 
                    };
                }
                'd' => {
                    input.clear();
                    println!("What file would you like to decrypt");
                    stdin().read_line(&mut input).unwrap();
                    input.pop();

                    file_decrypt(&input, &secret_key).unwrap();
                }
                _ => panic!("not e or d"),
            };
        }
    }
}

#[derive(PartialEq, Debug)]
struct User{
    username: String,
    password: String,
    secret_password: SecretKey,
}
impl User {
    fn new() -> User {
        let mut input = String::new();

        println!("Enter Username: ");

        stdin().read_line(&mut input).unwrap();
        input.pop();

        let user = input.clone();
        let mut first: String;

        loop {
            input.clear();
            println!("Enter Password: ");
            stdin().read_line(&mut input).unwrap();
            first = input.clone();
            
            input.clear();
            println!("Confirm Password: ");
            stdin().read_line(&mut input).unwrap();
            if first == input {
                break;
            }
        }

        first.pop();

        User {
            username: user,
            password: first,
            secret_password: SecretKey::generate(),
        }
    }
    fn to_tsv(&self) -> String {
        format!("{}\t{}\t{}\n",self.username,self.password, String::from_utf8_lossy(self.secret_password.unprotected_as_bytes()))
    }
    fn from_tsv(s: &str) -> User {
        let mut v = s.split_whitespace();
        User {
            username: v.next().unwrap().to_string(),
            password: v.next().unwrap().to_string(),
            secret_password: SecretKey::from_slice(v.next().unwrap().as_bytes()).unwrap(),
        }
    }
}

pub fn string_encrypt(secret_key: &aead::SecretKey) -> Result< (), Box<dyn Error>> {
    println!("Type string you'd like to be encrypted: ");
    let mut buf = String::new();
    stdin().read_line(& mut buf).unwrap();
    buf.pop();

    let text = aead::seal(&secret_key, buf.as_bytes())?;


    match write_to_file(&text, "string") {
        Ok(_) => Ok(()),
        Err(_) => Err("Writing Error")?,
    }
}

pub fn file_encrypt(secret_key: &aead::SecretKey) -> Result<(), Box<dyn Error>> {
    println!("Type file path you'd like to be encrypted: ");
    let mut buf = String::new();
    stdin().read_line(& mut buf).unwrap();
    buf.pop();    //gets rid of \n

    let contents = fs::read_to_string(buf.clone())?;
    let text = aead::seal(&secret_key, &contents.as_bytes())?; 

    match write_to_file(&text,buf.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Writing Error")?
    }
}

pub fn file_decrypt(s: &str, secret_key: &aead::SecretKey) -> Result<(), Box<dyn Error>> {
    let file = fs::read(s).expect("Reading problem");
    println!("{:?}", file);
    let open = aead::open(secret_key, &file).expect("Open problem");
    match write_to_file(&open, s) {
        Ok(_) => Ok(()),
        Err(_) => Err("Writing Error")?
    }
}