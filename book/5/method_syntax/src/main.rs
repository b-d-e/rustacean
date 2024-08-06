// in last section, we defined a function that operated on a struct, but wasn't explicitly related
// for things as specific as what we are doing here, it can be useful to define as a method
// instead

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// to define method on struct, start an impl (implementation) block
// everything in here only gets associated with the rectangle type
impl Rectangle {
    fn area(&self) -> u32 {
        // i.e. area is specific to rectangle
        // only parameter in signature needs to be self - OO style
        // &self actually gets parsed as self: &Self - it is an alias
        // methods must have self parameter first
        // note, still need to make a borrowed item with &
        // methods can also borrow self mutably!
        self.width * self.height
    }

    // can have a method with the same name as a field, e.g.
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        // takes another instance of a rectangle
        self.width > other.width && self.height > other.height
    }

    // can also have associated functions (which don't take self as a parameter)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// structs are allowed to use multiple impl blocks.
// there are times when this is useful (w.r.t generic types and traits, chapter 10), but for
// readability don't do it otherwise.

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("It is {} that the rectangle has width.", rect1.width());
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // often, setting method with same name as field means we just want to return the value - a
    // 'getter' which rust does not implement be default. this will be useful when we think about
    // public vs private methods in chapter 7

    // unlike C or C++, we don't need to think about whether we are applying a method to a value vs
    // a reference (. vs ->). Here, rust does automatic referencing a dereferencing) for methods
    // (and not much else). i.e., the two below lines are identical in compiled behaviour:
    // p1.distance(&p2);
    // (&p1).distance(&p2);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
