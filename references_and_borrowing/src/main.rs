
fn calculate_length(s : &String) -> usize {
    s.len()
}

fn borrow() {
    let s2 = String::from("hello");
    let len = calculate_length(&s2);
    println!("The length of '{}' is {}", s2, len);
}

fn change(some_string : &mut String)
{
    some_string.push_str(" world!")
}

fn mutable_vs_immutable_borrowing()
{
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);

    let rm = &mut s;
    println!("{}", rm)
}

fn dangle() -> String {
    let s = String::from("rustecean");
    s
}

fn main() {
    borrow();
    let mut s = String::from("hello");
    change(&mut s);
    mutable_vs_immutable_borrowing();
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}
