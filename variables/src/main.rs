use std::io;

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let x = x + 1;
    println!("The value of x is: {}", x);

    let x = x * 2;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS is: {}", MAX_POINTS);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess is: {}", guess);

    let x = 2.1;
    println!("The value of x is: {}", x);

    let y: f32 = 3.1;
    println!("The value of y is: {}", y);

    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);

    let t: bool = true;
    let f: bool = false;

    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    let tuple_example: (i32, f64, u8) = (123, 6.5464, 1);
    println!("tuple_example: {}, {}", tuple_example.0, tuple_example.1);

    let (x, y, z) = tuple_example;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let array = [1, 2, 3, 4, 5];
    println!("array: {}", array[0]);
    println!("array: {}", array[1]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("first element of a: {}", a[0]);

    let b = [1; 10];
    println!("second element of b: {}", b[1]);

    // How to handle errors when accessing invalid array indicies
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is {}",
        index, 
        element
    );

}
