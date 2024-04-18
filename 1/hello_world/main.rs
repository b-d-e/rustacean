fn main() { // always first function to be executed - entrypoint. no params. returns nothing.
            // params would go in parenthesis.
            // function body wrapped in curls brakcets {} - opening on same line as name, w/ space
    println!("Hello, world!");
    // rust style uses 4 spaces for indentation, NOT TAB (neovim / vs code should convert though)
    // println! calls a rust macro. without ! would call a function. macros in chptr 19, but follow
    // different rules to functions.
    // end expression lines with semi colon - indicates to compiler, rather than \n
}

// compile me with rustc main.rs 
// run me with ./main
// compiling and running are separate!
