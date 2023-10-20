#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // &self means self: &Self
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool{
        self.width >= rectangle.width && self.height >= rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}


fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectanlge is {} square pixels.", area_variables(width1, height1));

    let rect1_tuples = (30, 50);
    println!("The area of the rectanlge is {} square pixels.", area_tuples(rect1_tuples));

    let rect1_struct = Rectangle {
        width: 30,
        height: 50
    };
    println!("The area of the rectanlge is {} square pixels.", area_struct(&rect1_struct));
    println!("The area of the rectanlge is {} square pixels.", rect1_struct.area());

    println!("{:?}", rect1_struct); // or can use {:#?} to print pretty format
    dbg!(width1);
    dbg!(&rect1_struct); // will print to stderr
    dbg!(&rect1_struct.area());

    if rect1_struct.width() {
        println!(
            "The rectangle has a nonzero width; it is {}",
            rect1_struct.width
        );
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(32); // use of function without self
    dbg!(square);
}

fn area_variables(width: u32, height: u32) -> u32 {
    width * height 
}

fn area_tuples(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
