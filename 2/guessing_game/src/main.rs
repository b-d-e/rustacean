use std::io; // io library comes from standard (std) library - imported like this
// everything available by default is also from std, and is automatically loaded in from the
// prelude: https://doc.rust-lang.org/std/prelude/

fn main() { // programme entry point
    println!("Guess the number!"); // macro

    println!("Please input your guess.");

    // define (let) a variable which is mutable (mut) to hold guess
    // RUST VARIABLES ARE IMMUTABLE BY DEFAULT!
    // bind (=) to variable a new instance of String (from std - growable utf8)
    // use associated function of string (::)
    // call new function to make empty string. new is available for many types
    let mut guess = String::new();

    // call stdin function from io module, to handle user input
    // if we hadn't imported io, could do std::io::Stdin
    io::stdin()
        // call read_line method - pass reference (&) of variable to store response in
        // rust makes it really safe & easy to pass by reference :) 
        // references are mimmutable by default, so need '&mut guess' not '& guess'
        // chapter 4 explains references more thoroughly
        .read_line(&mut guess)
        // to handle failure, use expect - NOT EXCEPT!!!!
        // readl_line returns Result type (an enumeration - type that can take one of several
        // states). Here, states are Ok and Err - if Err, operation failed, raise exception below
        // if we don't call expect, program will compile with a warning
        .expect("Failed to read line");
        // note the above 3 lines of code are really one, with whitespace for readability
        // could write 'io::stdin().read_line(&mut guess).expect("Failed to read line");'
        

    // we can print variables with println! placeholders, {VARNAME}
    // 'little crab pincers that hold a value in place' 🦀
    // can use output of expression, e.g.:
    //     println!("x = {x} and y + 2 = {}", y + 2);
    println!("You guessed: {guess}");
}
