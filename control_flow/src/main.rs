fn main() {
    let number = 7;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2.");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("value of number is {}", number);

    let mut counter = 0;
    let mut result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result after looping is {}", result);

    while result != 0 {
        println!("{}", result);

        result -= 1
    }
    println!("Final result {}", result);

    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("the value is: {}", element);
    }

    println!("\n\nUsing .rev() to reverse an array");
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("Done!")
}
