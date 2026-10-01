use std::io;


fn main() -> () {
    let mut int_str = String::new();

    io::stdin()
        .read_line(&mut int_str)
        .expect("Provide a number please");

    let int_val: i32 = int_str.trim().parse().expect("Couldn't convert to int");
    let formatted_hex = format!("{int_val:X}");
    let formatted_binary = format!("{int_val:#b}");
    println!("As a decimal: {int_val}");
    println!("As a binary: {formatted_binary}");
    println!("As a hex: {formatted_hex}");
}