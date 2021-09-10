fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("heyheyhey@hellothere.com");

    println!(
        "user1:\n{}\n{}\n{}\n{}\n", 
        user1.email,
        user1.username,
        user1.active,
        user1.sign_in_count
    );

    let user2 = User {
        email: String::from("example@example.com"),
        username: String::from("example"),
        ..user1 // This allows user2 to inherit the other fields from user1
    };
    println!(
        "user2:\n{}\n{}\n{}\n{}\n", 
        user2.email,
        user2.username,
        user2.active,
        user2.sign_in_count
    );

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("black: {},{},{}", black.0, black.1, black.2);
    println!("origin: {},{},{}", origin.0, origin.1, origin.2);

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} sq pixels.",
        area(width1, height1)
    );

    // refactor with tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} sq pixels",
        area_t(rect1)
    );

    // refactor using Structs
    let rect2 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} sq pixels",
        area_struct(&rect2)
    );

    println!("rectangle: {:#?}", rect2);

    println!("Rectangle area method: {}", rect2.area());
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    println!(
        "Can rect1 hold rect2: {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "Can rect1 hold rect3: {}",
        rect1.can_hold(&rect3)
    );

    let sq = Rectangle::square(10);
    println!(
        "square height and width:\n{}\n{}",
        sq.height,
        sq.width
    );

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_t(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, second_rectangle: &Rectangle) -> bool {
        // if second_rectangle.height <= self.height && second_rectangle.width <= self.width {
        //     true
        // } else {
        //     false
        // }
        // simpler and equivalent expression
        self.width > second_rectangle.width && self.height > second_rectangle.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}