use std::{
    fs::{
        self,
        File,
    },
    io::{
        self,
        Read,
        Write,
        ErrorKind
    },
    path::PathBuf,
};

fn read_username_from_file(path : &str) -> Result<String, io::Error> {

    let f = File::open(path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_username_from_file2(path : &str) -> Result<String, io::Error> {

    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn try_read(path : &str) {
    
    let f = File::open(path);
    match f {
        Result::Ok(mut f) => {

            let mut text = String::new();
            if let Result::Ok(_) = f.read_to_string(&mut text)
            {
                println!("Text from file: \n{}", text);
            }
            else{
                println!("Error::Couldn't read text from file");
            }
        }
        Result::Err(error) => {

            println!("Error::Couldn't read the file\nCause:\n{:#?}", error);
            if ErrorKind::NotFound == error.kind() {
                //println!("Trying to create the file.");
                let mut f = File::create(path).unwrap_or_else(
                    |error| {
                    panic!("Problem creating the file: {:?}", error);
                    }
                );
                //let out = io::stdout();
                
                writeln!(f, "Some text has been written!").unwrap_or_else(
                    |error| { 
                        println!("Error writing to a newly created file! \n {:#?}", error); 
                });
            }
        }
    };
}

use std::error::Error;
use std::net::IpAddr;

fn parse_ip(ip_s : &str) -> Result<IpAddr, std::net::AddrParseError>
{
    let home : IpAddr = ip_s.parse()?;
    Ok(home)
}

fn main() {
    /*
    let path = "hello.txt";
    let scr = PathBuf::from(path);
    println!("Initial path: {:?}", fs::canonicalize(&scr));
    //try_read(path);
    let hello_txt = read_username_from_file("hello.txt");
    println!("{:?}", hello_txt.unwrap());
    */

    let ip = parse_ip("127.0.0.1");
    println!("{:?}", ip);
    let ip = parse_ip("127.0.0.1.0.1");
    println!("{:?}", ip);
}
