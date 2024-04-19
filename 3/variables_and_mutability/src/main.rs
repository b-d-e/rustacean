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
    // declaring variable with same name as previous - old var is shadowed by new
    // then compiler always sees second afterwards
    // shadow by repeating use of let keyword
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
        // inner scope creates a new variable - that does not live beyond scope
    }
    println!("The value of y in the outer scope is {y}");

    // versus using mut to create a mutable variable, using let again essentially creates a brand new variable with the old
    // variable's name. means we can do tings like change the type but keep the name. mut alone
    // wouldn't allow this.
    //
    let spaces = "     ";
    let spaces = spaces.len(); // by shadowing, we can convert same variable name from string to
                               // numerical type
}
