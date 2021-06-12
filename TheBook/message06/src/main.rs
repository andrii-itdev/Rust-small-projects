enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) 
    {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move{x:0, y:1} => println!("Move"),
            Message::Write(_) => println!("Write"),
            Message::ChangeColor(_,_,_) => println!("ChangeColor"),
            _ => println!("You're wrong!")
        }
    }
}

fn main() 
{
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number : Option<i32> = None;

    match some_number {
        Some(_) => println!("{}", some_number.expect("Wrong!")),
        None => println!("None")
    }
    match some_string {
        Some(_) => println!("{}", some_number.expect("Wrong!")),
        None => println!("None")
    }
    match absent_number {
        Some(_) => println!("{}", some_number.expect("Wrong!")),
        None => println!("None")
    }
}
