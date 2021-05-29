//use restaurant::eat_at_restaurant;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Random number: {}", secret_number);

    restaurant::eat_at_restaurant();
}