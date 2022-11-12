fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels",
        rect.area() /* area(&rect)*/
    );

    let small_rect = Rectangle {
        width: 20,
        height: 40,
    };

    let big_rect = Rectangle {
        width: 100,
        height: 80,
    };

    println!("rect can hold smallRect: {}", rect.can_hold(&small_rect));

    println!("rect can hold bigRect: {}", rect.can_hold(&big_rect));

    let sq = Rectangle::square(10);
}

#[derive(Debug)]
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
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// fn area(rectangle: &Rectangle ) -> u32 {
//     rectangle.width * rectangle.height
// }
