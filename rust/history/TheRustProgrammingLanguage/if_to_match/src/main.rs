fn main() {
    let number = 7;

    match number {
        number if number % 4 == 0 => println!("number is divisible by 4"),
        number if number % 3 == 0 => println!("number is divisible by 3"),
        number if number % 2 == 0 => println!("number is divisible by 2"),
        _ => println!("number is not divisible by 4, 3, or 2"),
    }
}
