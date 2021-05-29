use std::io;

fn num2ordinal(n : u8, upper_letter : bool) -> String
{
    let number = ( match n 
    {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        _ => "strange"
    }).to_string();
    
    if upper_letter { 
            let fl = number.chars().nth(0).expect("Can't take the first letter").to_uppercase();
            return format!("{}{}", fl, &number[1..]);
        }
    number
}

fn num2word(n : u8, upper_letter : bool) -> String
{
    let number = ( match n 
    {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "strange"
    }).to_string();
    
    if upper_letter { 
            let fl = number.chars().nth(0).expect("Can't take the first letter").to_uppercase();
            return format!("{}{}", fl, &number[1..]);
        }
    number
}

fn print_dayline(n : u8, first : bool)
{
    let word = num2ordinal(n, true);
    match n {
        1 => if first { println!("A partridge in a pear tree."); }
            else { println!("And partridge in a pear tree."); },
        2 => println!("{} turtle doves", word),
        3 => println!("{} French hens", word),
        4 => println!("{} calling birds", word),
        5 => println!("{} gold rings", word),
        6 => println!("{} geese a-laying", word),
        7 => println!("{} swans a-swimming", word),
        8 => println!("{} maids a-milking", word),
        9 => println!("{} ladies dancing", word),
        10 => println!("{} lords a-leaping", word),
        11 => println!("{} pipers piping", word),
        12 => println!("{} drummers drumming", word),
        _ => println!("")
    }
}

fn print_verse(n : u8) {
    let number = num2word(n, false);
    println!("On the {} day of Christmas my true love sent to me", number);
    for i in (1..n+1).rev() {
        print_dayline(i, n == 1);
    }
    println!();
}

fn main() {
    for i in 1..13{
        print_verse(i);
    }
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Couldn't read!");
}
