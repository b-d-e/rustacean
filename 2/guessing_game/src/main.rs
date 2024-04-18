use std::cmp::Ordering;
use std::io; // io library comes from standard (std) library - imported like this // use ordering enum (less, greater, equal variants)
                                                                                  // everything available by default is also from std, and is automatically loaded in from the
                                                                                  // prelude: https://doc.rust-lang.org/std/prelude/
use rand::Rng; // import Rng from random crate, we 'installed' from Cargo.toml dependencies
               // Rng trait defines methods that random number generators implement (see chapter 10)

fn main() {
    // programme entry point
    println!("Guess the number!"); // macro

    // call thread_rng function from rnd to get random generator local to current execution thread
    // and seeded by OS
    // gen_range function on this generator then gets number, takning range expression as argument
    // this range expression is form start..=end and is inclusive of lower and upper bounds
    // each crate has it's own docs, to tell you what methods is makes available!
    // run cargo doc --open to get docs and click rand in sidebar on left to view
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
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

        // convert guess to unsigned 32 bit number, so we can compare
        // call a few helper functions to tidy up the input - remove whitespace (esp as enter comes in
        // as \n into the variable). parse method converts string to anoter type.
        // guess already exists, so here we are shadowing with a new variable - this saves space ch3)
        // we tell compiler type anotation with colon
        // rust will infer from this type that secret_number should be u32 too!! very nice
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // we can print variables with println! placeholders, {VARNAME}
        // 'little crab pincers that hold a value in place' 🦀
        // can use output of expression, e.g.:
        //     println!("x = {x} and y + 2 = {}", y + 2);
        println!("You guessed: {guess}");

        // cmp method compares two values, called on anything comparable
        // takes reference to whatever you want to compare with
        // returns a variant of Ordering enum
        // use match expression (like python switch) to decide what to do based on what is returned
        // match express made up of 'arms' - pattern to match against and code to run if match
        // rust works through in turn - let's you handle all situations your code will encounter
        // ends after first successful match
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // arms end with comma, not sc
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! 🦀");
                break;
            }
        }
    }
}
