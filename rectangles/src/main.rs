fn main() {

    let r1 = Rectangle{
        width: 30,
        height: 50
    };
    let r2 = Rectangle{
        width: 10,
        height: 40
    };
    let r3 = Rectangle{
        width: 60,
        height: 45
    };
    println!("Can rect1 hold rect2? {}", r1.can_hold(&r2));
    println!("Can rect1 hold rect3? {}", r1.can_hold(&r3));

    println!(
        "The area of the rectangle is {} square pixels",
        rect_rect()
        //rect_tuple()
        //rect_regular()
    );
}

#[derive(Debug)]
struct Rectangle{
    width : u32,
    height : u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, rect : &Rectangle) -> bool {
        if rect.height < self.height 
           && rect.width < self.width
           {true} else {false}
    }
}

fn rect_rect() -> u32
{
    let rect = Rectangle::square(50);
    /*{
        width: 30,
        height: 50,
    };*/
    
    println!("rect is {:#?}", rect);
    rect.area()
    //area_rectangle(&rect)
}

/*
fn rect_regular() -> u32
{
    let width = 30;
    let height = 50;

    area(width, height)
}

fn rect_tuple() -> u32
{
    let rect1 = (30, 50);
    area_tuple(rect1)
}

fn area (width: u32, height: u32)-> u32{
    width * height
}

fn area_tuple(dims: (u32, u32)) -> u32{
    dims.0 * dims.1
}

fn area_rectangle(rectangle: &Rectangle) -> u32
{
    rectangle.width * rectangle.height
}
*/