#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn squeare(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = Rectangle::squeare(20);

    // println!("rect1 is {:#?}\n", rect1);
    dbg!(&rect1);

    println!("The area of the rect1 is {} square pixels.", rect1.area());
    println!("The area of the rect2 is {} square pixels.", rect2.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect1? {}", rect1.can_hold(&rect1));
}
