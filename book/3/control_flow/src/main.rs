fn main() {
    println!("Hello, world!");
    // control flow lets us add conditional execution. in rust, we most commonly use if and loops

    // if lets you branch on condition - provide condition, state, and optionally else state
    let number = 3;
    
    // condition - MUST be a bool - e.g. if number { } would break
    // unlike js / ruby / etc, will not auto convert non bool types to bool. must be explicit with
    // a condition
    if number < 5 {
        // true
        println!("condition was true");
    } else {
        // false
        println!("condition was false");
    }

    // we can handle multiple conditions with...
    // else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 }; // blocks of code evaluate to last expression in
                                                // them

    println!("The value of number is: {number}");


    // repetition with loops
    // rust provides several loops - loop, while, and for
    
    loop { 
        println!("again!");
        break // to run next line
    } // break out with ^C. programmatically break with 'break' keyword.
      // use 'continue' to skip over any remaining code in the iteration and go to next iteration
    
    let mut counter = 0;

    let result = loop { // assign output of loop to variable
        counter += 1;

        // keep going until condition is met
        if counter == 10 {
            // to return value from loop, add expression after break key word
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // if nesting loops, break and continue apple to the innermost scope you are in
    // but, you can sepcify a koop label which you can then use with break or continue to target
    // loop labels MUST begin with a single quote

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    // conditional loops with while
    // rather than evaluating condition within loop, do it in definition with a while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // looping through a collection with for
    // iterate over elements of collection (e.g. array)
    // we could do this manually with while and an index, e.g.:
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    // the above an be error prone - i.e. what if we index out of bounds - panic
    // also slow - compiler adds runtime code for check

    // but we can do it more concisely with for element in collection syntax
    for element in a {
        println!("the value is: {element}");
    }
    // this increases code safety and removed cjance of bugs
    // this means for loops are most commonly used rust loop construct
    // even in situations where in other languages, we'd use a while loop
    // i.e. above example with for:
    for number in (1..4).rev() { // use a range to generate collection, reverse it with rev method,
                                 // then
        println!("{number}!");
    }
}

// plenary ideas for this chapter
// - Convert temperatures between Fahrenheit and Celsius.
// - Generate the nth Fibonacci number.
// - Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
