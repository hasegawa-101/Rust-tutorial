use std::io;
mod calc;

fn main() {
    println!("num1を入力してください。");
    let mut num1_string = String::new();
    io::stdin().read_line(&mut num1_string).expect("Failed to read line");
    let num1: i32 = num1_string.parse().unwrap();
    println!("{}",calc::calc(num1,2));
}
