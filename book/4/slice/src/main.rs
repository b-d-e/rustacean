fn main() {
    // a slice references a contiguous subsequence of elements in a collection, rather than the
    // whole collection
    //

    let mut s = String::from("hello world");

    // our non slicing method is not robust to the string changing
    let word = first_word_no_slices(&s); // word will get the value 5
                                         // word is not connected to the state of s at all
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // rust has a solution - slices

    // a string slice is a REFERENCE to part of a string, e.g.
    let s = String::from("hello world");

    let hello = &s[0..5]; // references just part, not entire
    let world = &s[6..11]; // specify [start and end) indices
    let slice = &s[..2]; // can start from begining by dropping first number
                         // similarly, these two are equal:
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    // slicing must take place at UTF-8 character boundaries
    // if trying to slice a multibyte character, programme panics

    // now, first word with slices
    let word2 = first_word_slice(&s);

    // now, things like this will fail to compile!
    // s.clear(); // error! clear taking a mutable reference
    // but that cannot exist alongside the immutable reference word2
    println!("the first word is: {}", word2);

    //////////////////////
    // string literals as slices

    // recall, string literals instantiated like:
    let s = "Hello, world!";
    // type of s is &str - slice pointing to a specific point in the compiled binary. hence why
    // string literals are immutable.

    ///////////////////////
    // string slices as parameters
    // we can improve the signature of our first word function
    // to allow us to use it on &String and &str vales
    // fn first_word(s: &str) -> &str {
    // this flexibility is a result of deref coercions

    ///////////////////////
    // other slices
    // non string specific
    let a = [1, 2, 3, 4, 5]; // array

    let slice = &a[1..3]; // has type &[i32]

    assert_eq!(slice, &[2, 3]);
}

fn first_word_no_slices(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert to bytes to let us go through element by element

    for (i, &item) in bytes.iter().enumerate() {
        // iterate over array of bytes and enumerate to
        // get indices
        // destructure tuple with pattern to get index and
        // item
        if item == b' ' {
            // check if byte representation of space character found
            return i; // return positition
        }
    }

    s.len() // otherwise, return length of string
}

fn first_word_slice(s: &String) -> &str {
    // returns string reference
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // same method up to here to find i
            return &s[0..i]; // return slice of inout string
        }
    }

    &s[..]
}
