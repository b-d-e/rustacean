fn main() {
    // enums give us a way to say a value can be one of a possible set of values
    // e.g. Rectangle is in a set that also contains Circle and Triangle
    // in this demo, we'll think about IP v4 and v6
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
    // ^ attach data to each variant of enum directly
    // so no need for an extra struct
    // name of each enum type also becomes a constructor function, e.g. V4()
    // we can also have different variants of different types
    
    // IP addresses are also supported in the standard library, using enum structs
    // here though, we can create our own versions without conflict as we haven't brought std lib
    // into scope
    

}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// this is a custom data type we can use elsewhere in the code ^

// create instances of the variants like this
fn instantiate() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

// both are of the type IpAddrKind (different namespaces?)
// so can use in type signature
fn route(ip_kind: IpAddrKind) {}


// --------

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// four variants each with different types
// similar to defining lots of structs, but here they're all grouped together

impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

fn messaging() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

// -------
// another useful enum from the standard library - option

// option enum type encodes the scenario where a value could be something or could be nothing

// rust doesn't have the null feature - meaning there is no value. this is good, and prevents
// unintended bugs.
// but the concept null is trying to express is still useful. we should be able to represent a
// value as invalid or absent

// standard library defines enum Option<T> as follows
// enum Option<T> {
//    None,
//    Some(T),
// }

// it is so useful it is included in the prelude, and doesn't need to be explicitly brought into
// scope
fn options() {
    let some_number = Some(5);
    let some_char = Some('e'); // rust infers these types as value is assigned

    let absent_number: Option<i32> = None;
}

// Option<T> is better than null because Option<T> and T are different types. The compiler won't
// let us use an Option<T> where it shouldn't be.
