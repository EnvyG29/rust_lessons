use std::fs::File;
use std::io::{self, Read};

fn main() {
    //    panic!("ALAAAAAAARRRMM!!!!!!!!");
    fn1();
    println!("{:#?}", fn2());
}

fn fn1() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:#?}", error),
    };
}

fn fn2() -> Result<String, io::Error> {
    let username_from_file = File::open("hello.txt");

    let mut username_file = match username_from_file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
