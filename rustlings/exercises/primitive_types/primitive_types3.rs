// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let a = [0,3,4,2,4,2,4,2,245,2,34,34,234,234,234,324,234,234,3,23,3,324,324,324,34,34,34,34,43,42,34,234,324,324,324,32,432,432,432,4324,324,324,324,324,324,324,324,3,34,324,324,324,324,234,234,324,324,32,432,432,432,432,4,324,324,324,324,324,324,234,324,324,324,324,34,4,67678,768,678,768,678,768,768,768,876,867,867,867,867,8,768,678,678,678,678,67,867,876,876,8,678,678,678,678,678,8,678,678,678,678,678,678,678,678,678,678,678,678,678,678,768,678,768,5]; // there's probably a nicer way to do this :/

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
