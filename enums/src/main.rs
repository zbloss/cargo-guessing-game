fn main() {
    println!("Hello, world!");

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let some_number = Some(5);
    let some_string = Some("a simple string");
    let no_number: Option<i32> = None;

}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}