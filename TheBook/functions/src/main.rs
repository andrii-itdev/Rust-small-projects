

fn main() {
    let z = 5;
    let y = { 
        let x = 3;
        x + 1
    };
    println!("The value of z: {}  y: {}", z, y);
}
