// main function is entry point of pogramme 
fn main() {
    println!("Hello, world!");
    // call another function like this:
    another_function(); // note, doesn't have to come after the function def, and no concept of
                        // declaration. just needs to be defined in a scope the caller can see
    and_another(5); // pass argument
                    
    print_labeled_measurement(5, 'h'); // pass several arguments 
}

fn another_function() { // we use snake case in function and variable names
    println!("Another function.");
}

fn and_another(x: i32) { // specify parameter, with it's type annotation
    // you MUST provide type annotation in function parameters - this means compiler rarely forces
    // you to specify them elsewhere - it can use these to infer
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) { // two parameters, each with their own
                                                             // (different) type annotation)
    println!("The measurement is: {value}{unit_label}");
}


// rust is made up of statements and expressions
// statements are instructions that perform some action and do NOT return a value, i.e. let-ting a
// value
// expressions ecaluate to a reultant value, e.g. a function definition
// compiler will error if you try and assign to something that does not return a value. e.g.
// let x = (let y = 6); will not work
//
// expression always evaluate to a value - and are most of what we'll write in rust
// e.g. 5 + 6
// calling a function or a macro is an expression. a new scope block is an expression, returning a
// value... e.g.
fn example() {
    let y = { // this {} is an expression
        let x = 3;
        x + 1 // note no semicolon - that would turn it into a statement at it wouldn't return a
              // value... be careful with this
    }; // block expression evaluates to 4

    println!("The value of y is: {y}");
}


// returning values from functions
// don't name return values, but MUST declare their type after an arrow in the function def
// value retruend is synonymous with the value of the final expression
// can return early with return keywork and specifying a value. generally, we return implicitly. 

fn five() -> i32 { // specify return type with -> notation
    5 // this is perfectly valid. 
}

fn notmain() {
    let x = five(); // return value of function to init variable

    println!("The value of x is: {x}");
}

fn alsonotmain() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // if there were a ;, it would break (error mismatched type, because function would
          // return () not i32). compiler suggsets removing the ;.
}
