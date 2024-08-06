fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("Area of rectang is {} square units.", area(width1, height1));

    // this works, but is not clearly written. nowhere in the programme is is properly defined that
    // the parameters are related. it would be more manageble to group them together
    // this could be done with a Tuple:

    let rect1 = (30, 50);

    println!("Area is {}.", tupleArea(rect1));

    // this is better - to a certain degree. whilst it is good we just pass one arg, it's confusing
    // given the tuple elements to not have names and could arbitarily be interchanged.
    // the indexing is not self-documenting, and is unclear, making it easier to introduce errors
    // structs add more meaning:

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "Area (struct method) is {} square units.",
        structArea(&rect1)
    );

    // why is this better?
    // - elements are named
    // - only one parameter is passed
    // - can be borrowed immutably

    // structs also let us add functionality with derrived traits, e.g.
    println!("rect1 is {:?}", rect1); // :? tells rustc to use debug format of printing

    // we can also use :#? to split struct over several lines, e.g.
    println!("rect is is {:#?}", rect1);

    // we can also print debug info with the dbg! macro - unlike println! (which uses a reference)
    // this takes ownership of expression, prints file and line number along with expression, and
    // then returns ownership
    dbg!(&rect1);

    // can also wrap indicdual struct elements in a debug expression - as value is ultimately
    // yielded, this doesn't alter assignment functionality
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // in addition to debug, rust implements lots of other useful traits - see Appendix C of the
    // book
    // can also implement our own traits with custom behaviour (chapter 10)
}

#[derive(Debug)] // rust supports printing debug info, but has to be explicitly enabled - off by
                 // default
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tupleArea(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn structArea(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
