use std::io;

fn f2c(tem : f64) -> f64 { (tem - 32.0)*5.0/9.0 }

fn main() {
    let mut inp = String::new();
    println!("Enter some fahrenheits: ");
    io::stdin().read_line(&mut inp).expect("Error reading!");
    let frht : f64 = inp.trim().parse().expect("Error parsing!");
    let cel : f64 = f2c(frht);

    println!("Here is your Celsius: {}", cel);
}
