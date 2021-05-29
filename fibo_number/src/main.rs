
fn fibo(n : u128) -> u128 {
    let mut value : u128 = 1;
    let mut prev : u128 = 0;
    if n == 0 {return prev}
    if n == 1 {return value}
    for _a in 1..n {
        let temp = value;
        value = value + prev;
        prev = temp;
    }
    value
}

fn main() {
    let n = 186;
    println!("fibo of {} = {}", n, fibo(n));
    /*for n in 0..20{
        println!("fibo of {} = {}", n, fibo(n));
    }*/
}
