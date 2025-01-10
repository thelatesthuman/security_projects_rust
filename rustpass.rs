use std::env;
use std::fs::File;
use std::io::{Read, Write};
use sha2::{Sha512, Digest};
use hex;


// Creates shadow file if doesn't exist, then writes username and password to it
fn file_write(file: &str, 
              username: &str, 
              password: &str) -> std::io::Result<()> {
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(file)?;
    if check_username(username) == true {
        writeln!(&mut file, 
                 "{user}:{pass}", 
                 user=username, 
                 pass=encrypt(&password))?;
        println!("Username and password written to shadow");
    }
    else {
        println!("Username is not available");
    }
    Ok(())
}

// Reads from file and returns the contents as a string
fn file_read(file: &str) -> String {
    let file = File::open(file);
    let mut contents = String::new();
    let _ = file
        .expect("Issue reading from shadow file")
        .read_to_string(&mut contents);
    contents
}

// Encrypt password with Sha512 hashing
// TODO: add random salt to increase security
fn encrypt(password: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();
    let hash_enc = hex::encode(hash);
    hash_enc
}



// Compare username and password to entries in shadow file
fn authenticate(username: &str, password: &String) -> bool {
    let file = file_read("shadow");
    let mut authenticated: bool = false;
    for line in &file.lines().collect::<Vec<_>>() {
        if username == line.split(":").collect::<Vec<_>>()[0] {
            if encrypt(password) == line.split(":").collect::<Vec<_>>()[1] {
                authenticated = true;
            }
        }
    }
    authenticated
}

// Username check to make sure a username isn't taken before writing to shadow file
fn check_username(username: &str) -> bool {
    let file = file_read("shadow");
    let mut username_avail: bool = true;
    for line in &file.lines().collect::<Vec<_>>() {
        if username == line.split(":").collect::<Vec<_>>()[0] {
            username_avail = false;
        }
    }
    username_avail
}

fn main() {
    let args: Vec<String> = env::args().collect();

// TODO: change these conditionals to match cases
    if args[1] == "file_write" {
        let _ = file_write("shadow", &args[2], &args[3]);
    }
    else if args[1] == "file_read" {
        let _ = print!("{}", file_read("shadow"));
    }
    else if args[1] == "authenticate" {
        let auth = authenticate(&args[2], &args[3]);
        //println!("{:#?}", auth);
        if auth == true {
            println!("You are authenticated");
        }
        else {
            println!("You are NOT authenticated");
        }
    }
}
