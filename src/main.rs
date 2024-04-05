use std::io;

fn convert_to_int(s: & String) -> i32 {
    let x = s.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Failed to read number 1");

    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Failed to read number 2");

    if convert_to_int(&number1) > convert_to_int(&number2) {
        println!("{} is greater than {}", number1, number2);
    } else {
        println!("{} is greater than {}", number2, number1);
    }
}
