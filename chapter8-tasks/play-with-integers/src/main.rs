


fn read_integers() -> Vec<i32> {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Couldn't read from console!");
    let snippets = input
        .split(|ch| ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' || ch == ',' || ch == ';')
        .filter(|v| v != &"" && v != &" " && v != &"\n" && v != &"\r");
    //println!("Snippets: {:#?}", snippets);
    let mut integers : Vec<i32> = Vec::new();
    for s_int in snippets {
        println!("Value: '{}'", s_int);
        let i = s_int.parse::<i32>()
            .expect(
                &format!("You've entered not a valid number: {}", s_int)
            );
        integers.push(i);
    }
    integers
}

fn mean_value(v : &Vec::<i32>) -> i32 {
    let mut sum : i32 = 0;
    for i in v {
        sum += i;
    }
    sum / (v.len() as i32)
}

fn median_value(v : &Vec::<i32>) -> i32 {
    v.clone().sort();
    let m = v.len() / 2;
    *v.get(m).expect("No values in the array")
}

use std::collections::HashMap as HM;

fn mode_value(v : &Vec::<i32>) -> HM::<i32, i32> {
    let mut mode : HM<_,_> = HM::new();
    for item in v {
        let entry = mode.entry(*item).or_insert(0);
        *entry += 1;
    }
    mode
}

fn print_mode(values : HM::<i32,i32>) {
    for (k,v) in values {
        println!("{} - has {} occurences", k, v);
    }
}

fn main() {
    println!("Please enter integer numbers separated with ' ' or ',' ");
    let integers = read_integers();
    println!("Your numbers: {:?}", &integers);
    println!("Mean: {}", mean_value(&integers));
    println!("Median: {}", median_value(&integers));
    println!("Your numbers: {:?}", &integers);
    let mode_values = mode_value(&integers);
    print_mode(mode_values);
}
