use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::prelude::*;

fn match_regular() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => { println!("It's ok!"); file },
        Err(error) => { println!("It's Not ok!"); panic!("Problem opening the file: {:?}", error)}
    };
}

fn match_different_errors() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Cannot create the file {:?}", e)
            },
            any_error => {
                panic!("Problem opening the file: {:?}", any_error)
            }
        }
    };
}

fn match_different_errors_closure(path : &str) -> File {
    let f = File::open(path).unwrap_or_else(
        |error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(path).unwrap_or_else(
                    |error| {
                        panic!("Problem creating the file: {:?}", error);
                    }
                )
            }
            else {
                panic!("Problem opening the file: {:?}");
            }
        }
    );
    f
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    //match_regular();
    //match_different_errors();
    let mut fi = match_different_errors_closure("hello.txt");
    write!(&mut fi, "Written text").unwrap_or_else(|err| {
        //writeln!("Error!");
    });
}
