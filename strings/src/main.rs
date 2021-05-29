

fn iterate_over_string(s : &String)
{
    for (i, &item) in s.as_bytes().iter().enumerate() {
        println!("{} :: {} :: ", i, item);
    }
}

fn iterate_over_str(s : &str)
{
    iterate_over_string(&s.to_string());
}

fn characters(s : &String)
{
    for c in s.chars(){
        println!("{}", c);
    }
}

fn combining() -> String {
    let s1 = "Hello".to_string();
    let s2 = "world".to_string();
    let s3 = "!!!".to_string();
    s1 + &s2 + &s3
}

fn tic_tac_toe() -> String {
    
    let s1 = "Tic".to_string();
    let s2 = "Tac".to_string();
    let s3 = "Toe".to_string();
    format!("{}-{}-{}", &s1, &s2, &s3)
}

fn string_slices(){
    let s = "mad dog!";
    println!("{}", &s[0..2]);
}

fn try_hindi(){
    let hindi = "дनमस्ते";
    iterate_over_str(&hindi);
    println!("{}", hindi);

    println!("Cyrylic slice: {}", &hindi[0..2]);
    println!("Hindi slice: {}", &hindi[2..5]);
    println!("{}", &"hindi"[1..2]);
}

/*

* Important information
! Hey, this code isn't working
? Is this still working
TODO This function is not finished

*/

fn main() {
    //println!("Combined string: {}", combining());
    //println!("TTT: {}", tic_tac_toe());
    //let item = "ABC".get(0x1usize);
    //match item {
    //    Some(it) => println!("Indexing: {}", it),
    //}
    //string_slices();
    //try_hindi();
    characters(&"नमस्ते".to_string());
}
