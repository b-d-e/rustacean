fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // use ampersand to pass as reference

    println!("The length of '{}' is {}.", s1, len);

    change(&s1);

    let mut s2 = String::from("hello"); // declare variable as mutable
    mutchange(&mut s2); // &mut is type for mutable reference

    println!("The value is now '{}'", s2);
}

fn calculate_length(s: &String) -> usize { // make sure there's an ampersand here too!
    s.len()
        // value pointed to by reference goes out of scope
        // but because it doesn't have ownership over what it refers to, it is NOT dropped
        // never owned it, so do not need to return it!
}


// we can dereference with operator * - more in chapter 8
//
// we call the action of creating a reference 'borrowing'
//
// if we try modifying something we borrow, w'll hit a compulation error, e.g...

fn change(some_string: &String) {
    // some_string.push_str(", world");
}
//gives "`some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable"

// references are immutable by default...
//
// let's try making it mutable

fn mutchange(some_string: &mut String) {
    some_string.push_str(", world");
}



// big restriction of mutable references
// if you have one mutable reference, you can have NO other references to that variable
//
// e.g. this would fail:
//
//   let mut s = String::from("hello");

//   let r1 = &mut s;
//   let r2 = &mut s;
//   println!("{}, {}", r1, r2);
//
//  so, we have mutability, but only in a very controlled way. 
//  this is a pretty different paradigm of working to most languages
//
//  the benefit is we can prevent race conditions at compile time
//
//  we can use curly brackets to make a enw scope, allowing multiple mutable references, so long as
//  they aren't _simultaneous_

fn multimut() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}


// we also cannot have immutable references together with mutable ones. similar error produced.
//
//
// a references scope starts when it is introduced and it continues through to the last time it is
// used. so something like this is fine:

fn allowed() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// normally, we have to be careful with DANGLING POINTERS
// this references a location that may have been given to something else
// but again, rustc comes to the rescue and won't let us have these - e.g. 

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

// will error
// solution - just return 's', not '&s'

