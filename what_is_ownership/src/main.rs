
fn takes_ownership(some_string : String) {
    println!("takes_ownership: {}", some_string);
}

fn makes_copy(some_integer : i32)
{
    println!("makes_copy: {}", some_integer);
}

fn taking_ownership_and_copying()
{
    let s = String::from("hello ");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
    println!("ownership_and_functions: {}", x);
}


fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn giving_ownership()
{
    println!("giving ownership");
    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = String::from("hello");
    println!("{}", s2);
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);
}

fn calculate_length(s : String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn take_and_give_ownership() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of {} is {}.", s2, len);
}

fn main() {
    let s1 = String::from("Hello!");
    let s2 = &s1;
    let s3 = s1.clone();
    println!("{}", s2);
    println!("{}", s1);
    println!("{}", s3);

    println!("s1 == s2 = {}", &s1 == s2);
    println!("s2 == s3 = {}", s2 == &s3);
    println!("s3 == s1 = {}", s3 == s1);

    taking_ownership_and_copying();
    giving_ownership();
    take_and_give_ownership();
}
