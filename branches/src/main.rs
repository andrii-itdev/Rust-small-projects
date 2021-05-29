
fn value(bl : bool, xl : bool) -> i32
{
    if bl && true {5} else if xl {10} else {4}
}

fn multiple(){
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn a_loop(){
    let mut i: i128 = 0;
    let result = loop
    {
        println!("again! {}", i);
        i += 1;
        if i == 10 {break i * 12;}
    };
    println!("The resulting value = {}", result);
}

fn if_expression(){
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {0}", number);
}

fn if_simple_statement(){
    let number = 3;

    if number < 5 {
        println!("Condition was True");
    }
    else{
        println!("Condition was False");
    }
}

fn while_with_array(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
fn for_loop_over_array()
{
    let arr = [10,20,30,40,50];
    for element in arr.iter()
    {
        println!("Item: {}", element);
    }
}
fn for_loop_countdown()
{
    let x = 1..5;
    for number in x.rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn main() {
    if_simple_statement();

    println!("Value: {}", value(false, false));

    multiple();

    if_expression();

    a_loop();

    while_loop();

    while_with_array();

    for_loop_over_array();

    for_loop_countdown();
}
