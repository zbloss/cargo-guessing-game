fn first_function() {
    println!("Hey this is the first function!");
}

fn main() {
    println!("Hello, world!");
    another_function(5);
    first_function();
    let data = five();
    println!("data is: {}", data);
    println!("Result of plus one: {}", plus_one(3));
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}