fn main() {
    let rect1 = Rectangle {
        width: 30_f64,
        height: 50_f64,
    };
    let rect2 = Rectangle {
        width: 10_f64,
        height: 40_f64,
    };
    let rect3 = Rectangle {
        width: 60_f64,
        height: 45_f64,
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    println!("The area of the rectange is {}.", (&rect1).area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Square or size 5: {:#?}", Rectangle::square(5_f64));
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}