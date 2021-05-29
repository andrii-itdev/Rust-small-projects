

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn _placeholder()
{
    let some_u8_value = 0u8;
    match some_u8_value {
        0 => println!("Zero"),
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        6 => println!("Six"),
        7 => println!("Seven"),
        8 => println!("Eight"),
        _ => (),
    };
}

fn fn_match(){
    let five = Some(5);
    let six = plus_one(five);
    println!("{:#?}", six);
    let none = plus_one(None);
    println!("{:#?}", none);

    _placeholder();
}

fn if_let(){
    let some_u8_value = Some(3);
    /*match some_u8_value {
        Some(3) => println!("Three"),
        _ => ()
    };*/
    
    if Some(3) == some_u8_value {
        println!("IF LET Three")
    }
}

fn main() {
    fn_match();
    if_let();
}
