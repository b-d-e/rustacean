// every value in rust has an owner
//
// there can only be one owner at a time
//
// when the owner goes out of scope, the value will be dropped
//
fn main() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
                         // s is a string literal reference, of known fixed size
                         // do stuff with s
                         //
                         // we know contents at compile time, so text hardcoded into executable code
                         // fast and efficient
                         //
    }

    // let's consider a value stored on the heap
    // the String type (new to us) is one of these data types
    // String is covered in more depth in chapter 8
    // you can create a String from a string Literal with something like:
    let mut s = String::from("hello");
    // double colon :: namespaces from function under String type. more in chapters 5 and 7
    // this kind of string can be mutated (mutable, below), but doesn't have to be
    s.push_str(", world!"); // push_str() appends a literal to a String

    // to support a string type, that can be mutated,etc, we need to allocate an amount of memory
    // on the heap - this is unknown at compile time. so...
    // - memory must be requested from allocator at run time - done by us when we call String::from
    // - need way of returning memory to allocator when we're done with the string - different. no
    //   garbage collector in rust. equally, we don't have to explicitly free in rust.
    //   memory is automatically returned once the variable that owns it goes out of scope
    //
    //
    println!("{}", s); // This will print `hello, world!`

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no
      // longer valid
      // when goes out of scope, rust calls 'drop' function automatically at }
      // this is called Resource Acquisition Is Initialization (RAII) in C++

    // this has significant impacts on how we right rust. in complex programmes, can lead to
    // unexpected behavious where we want to have multiple variables use the data we've allocated
    // on the heap. for instance...

    // multiple variables can interact with the same data in different ways
    let x = 5;
    let y = x;
    // because integers are fixed known size, these 5 values are pushed to the stack

    // what about the String equivilant:
    let s1 = String::from("hello");
    let s2 = s1;
    // initially looks similar, easy to assume same behaviour (s2 copies value and binds to
    // memory). this isn't what is actually happening though

    // String is made of three parts - pointer to memory that holds content, a length, and a
    // capacity. this data is stored on the stack. this points to a location on the heap

    // capacity is amount String has received from allocator, length is amount currently used
    // when we assign s1 to s2, the String data is copied - that is, the pointer, length, and
    // capacity. we do NOT copy data on the heap.

    // mentioned drop is called when variable goes out of scope, cleaning up heap memory
    // but, when we copy BOTH s1 and s2 will try to free the same memory.
    // this is a 'double free' error and can lead to corruption, and security vulnerabilities
    // to ensure memory safety, after 'let s2 = s1;', rust considers s1 as NO LONGER VALID!
    // can't used after s2 is created - get invalidated reference (telling you where it moved)

    // so it's NOT a shallow copy. it's a 'move'
    // design choice implied here - rust will never auto create 'deep' copy
    // so 'automatic' copying can be assumed to be inexpensive

    // if we DO want to Deep Copy the heap data of the string (not just stack), we can use clone
    // method. here's what it looks like:
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // clone is not needed for our integer example, because fixed size stored on the stack. there'd
    // be no practical difference between a deep and a shallow copy
    //
    // rust has annotation called the Copy trait, can place on types stored on the stack. if
    // implemented, variables do not move but are trivially copied, like ints.
    // rust won't let us annotate with Copy if type or any of it's parts have the Drop trait

    // generally, simple scalar values can implement Copy. nothing that requires allocation or is
    // some form of resource can implement Copy.
    // some types that can - u32, bool, f64, char
    // tuples can ONLY if they only contain types that also implement Copy.
    //}

    //fn main() {

    // ownership and functions
    // passing a variable to a function will move or copy, just as assignment does
    // below we see where variables go in and out of scope
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
                        // if we try to use s here, get compile-time error

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
                   // can use x here
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// returning values can also transfer ownership
fn not_main() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

// it's a pain to have to give then reclaim ownership every time we pass into a function.
// rust has a solution to this - references. we'll see this in the next subchapter.
