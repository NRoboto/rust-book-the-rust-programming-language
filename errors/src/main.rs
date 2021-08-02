use std::{fs::File, io::{ErrorKind, Read}, io};

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Could not create file: {:?}", e),
            },
            _ => panic!("Could not open file: {:?}", error),
        }
    };

    let username = read_username_from_file().expect("Could not read username");

    let my_guess = Guess::new(10);
    println!("my_guess = {}", my_guess.value());
    
    let my_guess2 = Guess::new(101); // Panics
    println!("my_guess = {}", my_guess2.value());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be between 1 and 100, got {}", value);
        }

        Guess {
            value,
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}