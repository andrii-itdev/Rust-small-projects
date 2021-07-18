
use std::fmt::Display;

struct  Pair<T> {
    x : T, 
    y : T
}

impl<T> Pair<T> {
    fn new (x: T, y : T) -> Self {
        Self { x, y }
    }
}

impl<T : Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        }
        else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

impl<T> std::fmt::Display for Pair<T>
where T : Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pair({}, {})", self.x, self.y)
    }
}

fn main() {
    let pair = Pair::new(30, 4);
    pair.cmp_display();
    println!("{}", pair);
}
