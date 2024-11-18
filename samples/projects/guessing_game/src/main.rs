//bring the input and output library in scope
use std::io;

fn main() {
    println!("Guessing the number!");
    println!("Please enter your guess");

    //guess the rust mutable variable. Means program allow us to change its value.
    //FYI by default rust, variables are immutable, means once we assign value to variable, the value would not change.
    let mut guess = String::new();

    //stdin used to handle the standard input for your terminal
    // the function read_line, use to get input from the user.
    //complete line can be re writtern this way also
    //io::stdin().read_line(&mut guess).expect("Failed to read line");
    //here, long line is divided in multiple line to improve the code readability

    //not if you don't call the .expect, the program will compile with warning message.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess number {}", guess);
}
