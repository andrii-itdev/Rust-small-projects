use std::io;
use std::cmp::Ordering;

fn readline() -> io::Result<String> {
    let stdin = io::stdin();
    let mut ret_str = String::new();
    if let Err(err) = stdin.read_line(&mut ret_str) {
        return Err(err);
    }
    else {
        return Ok(ret_str);
    }
}

fn guessing_game(secret_number : &i32) -> io::Result<()> {
    loop {
        let guess = readline()?;

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("More"); },
            Ordering::Equal => { 
                println!("Horray! You've guessed it correctly!");
                return Ok(());
            },
            Ordering::Greater => { println!("Less"); },
        }
    }
}

use rand::Rng;

fn main() -> io::Result<()> {
    println!("This is a guessing game, enter a number: ");
    let random_number = rand::thread_rng().gen_range(1, 101);
    guessing_game(&random_number)
}
