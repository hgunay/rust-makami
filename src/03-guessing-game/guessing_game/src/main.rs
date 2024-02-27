// This library is included with Rust; it’s not part of the core language.
// The std::io library comes from the Rust standard library, which is part of the standard distribution of Rust that we installed in Chapter 1.
// The io library is one of the libraries included in the standard library that provides input and output functionality.
// The io library comes from the standard library, known as std:: This library is included with Rust; it’s not part of the core language.
use std::io;

// The fn syntax declares a new function; the parentheses, (), indicate there are no parameters; and the curly bracket, {, starts the body of the function.
fn main() {
    // The println! macro prints a string to the screen.
    println!("Guess the number!");

    println!("Please input your guess.");

    // The let keyword is used to create a new variable.
    let mut guess = String::new();

    // The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
    // The read_line method on the standard input handle is used to get input from the user.
    // The except method can be called on the Result to crash the program and display the message that you passed as an argument.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
