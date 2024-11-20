use std::io;

fn main() {
    println!("control flow!");

    if_else(5);
    if_else(10);
}

fn if_else(value: u8) {
    if value > 5 {
        println!("input value {} is more than 5", value);
    } else if value == 5 {
        println!("input value is equal to 5");
    } else {
        println!("input value {} is less than 5", value);
    }
}
