#[derive(Debug)] // for print
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn construct_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // use tuple
    let rect1 = (30, 50);

    println!("Area of rect1 = {}", area_tuple(rect1));
    
    // use struct
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    
    println!("Area of rect2 = {}", area_struct(&rect2));

    // print struct using #[derive(Debug)]
    println!("rect2 = {:#?}", rect2); // {:?}
    println!("rect22 = {:#?}", rect2); // {:?}

    // print struct using dbg!
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);

    // method
    println!("area method in Rectangle = {}", rect3.area());
    println!("rect3 can hold rect2? {}", rect3.can_hold(&rect2));
    println!("rect2 can hold rect3? {}", rect2.can_hold(&rect3));

    // associated functions
    let square = Rectangle::construct_square(10);
    println!("square = {:?}", square);
}

fn area_tuple(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}