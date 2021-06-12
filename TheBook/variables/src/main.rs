

fn main() {

    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * y;
    println!("The value of y is {}", y);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The number of spaces: {}", spaces);

    let x = 2.0;
    let y : f32 = 3.0;

    let sum = 3.5 + 4.0;

    println!("x: {}  y: {}  sum: {}", x, y, sum);

    let bug : char = '🐞';
    println!("{}", bug);

    let tup : (i32, f64, u8) = (500, 5.6, 2);
    let (_z,_x,c) = tup;

    println!("{} {}", tup.1, c);

    let months : [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("4'th month: {}", months[4]);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let index : usize = 0;
    println!("Value {}: {}", index, a[index]);
}
