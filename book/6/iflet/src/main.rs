fn main() {

    // if let syntacx lets us combine the expressions in a less verbose way
    // without it, we'd do something like this
    let config_max = Some(6u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    
    // instead we can do this
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // this is muchmore concise
    // works similarly to a match - takes pattern and expression seperated by =
    // not sure i fully understandthis??
    
    // it is syntactic sugar for a match that runs code when the value matches one pattern only and
    // ignores all other values
    
    // can conclude an if let with an else - this is what would go in the _ case in a match

}
