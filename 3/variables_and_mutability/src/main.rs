//fn brokenmain() {
//    // this won't work
//    let x = 5;
//    println!("The value of x is: {x}");
//    x = 6; // 'can't assign twice to immutable variable'
//    println!("The value of x is: {x}");
//}

fn main() {
    println!("Variables and Mutability \n --------------");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Constants \n ------------------");

    // why use constant not immutable variable?
    // not allowed to use mut with constants - ALWAYS immutable
    // use const, not let, to declare them
    // constants MUST be type annotated (here as unsinged 32 bit number)
    // can be declared in any scope
    // constants can only be set to a cosntant expression, not the output of an expression that
    // could only be computed at run time. below is an expression, but it can be computed in
    // advance so is okay!
    // naming convention -all upper case with underscores between words
    // compiler evaluates this expression at compile time, so no performance loss at runtime
    // valid for entire time a program runs, in the declaration scope
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
    println!("Shadowing \n ------------------");
}
