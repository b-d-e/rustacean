fn main() {
    // every value in rust is of a data type
    // below we explore two data type subsets: scalar and compound
    // rust is statically typed, so must know all types at compile time
    // compuler can often infer what is wanted based on the value and how it is used
    //
    // when many types are possible, we must add an annoyation like this:
    let guess: u32 = "42".parse().expect("Not a number!");
    // without the u32 type annoyation, the compuler will give error 'type annotations needed'

    println!("Scalar types\n--------------");
    // represents a single value. rust has 4 primary scalar types: integers, floating point
    // numbers, booleans, and characters.
    println!(
        "Integer types\nLength	Signed	Unsigned
    \n8-bit	i8	u8
    \n16-bit	i16	u16
    \n32-bit	i32	u32
    \n64-bit	i64	u64
    \n128-bit	i128	u128
    \narch	isize	usize"
    );

    // each variant needs to be signed or unsigned, and has an explicit size (i.e., is it possible
    // to be negative, and how many bits it takes up)
    // signed variants store -2^(n-1) -> 2^(n-1)-1 range, unsigned stores 0->2^n - 1
    // signed stored using two's complement
    //
    // isize and usize depends on architecture of machine (64/32 bit system)
    //
    // integer literals can take any of the following forms:
    println!(
        "Integer literals\n\nNumber literals	Example
    \nDecimal	98_222
    \nHex	0xff
    \nOctal	0o77
    \nBinary	0b1111_0000
    \nByte (u8 only)	b'A'"
    );
    // so which to use? default is i32. use isize or usize primarily when indexig a collection
    //
    // beware of integer overflow. in debug mode, overflow will cause a panic at run time.
    // in release mode, if overflow occurs, rust performs two's complement wrapping - wraps around
    // to the minimum value the type can hold. this is obviously still not good.
    //
    // standard library provides 3 families of methods to handle overflow for primative numerics:
    // wrap in all modes, with wrapping_* methods (e.g. wrapping_add)
    // return None with checked_*
    // return value and boolean indeicating if overflow happened (overflowing_*)
    // saturate at values max or min with saturating_*

    // floating points
    // two primitative types for FPs - f32 and f64, 32 and 64 bits in size. default is f64 - as
    // fast on modern CPUs and capable of greater precision
    // ALL FPs are signed
    //
    // follow IEEE-754
    let x = 2.0; // f64 - default

    let y: f32 = 3.0; // f32

    // numeric operations

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // boolean type
    // lower case word
    let t = true; // implicit type annotation

    let f: bool = false; // explicit type annotation

    // character type
    // char is most primitive alphabetic type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // char type is 4 bytes and represents a unicode scalar value - far more powerful than ascii
    // specify char literals with 's'ingle quotes. string literals use "double quotes"
    // a character isn't really a true concept in rust - they're just scalar values U+0000 to
    // D+D7FF and U+E000 to U+10FFFF. more in chapter 8
    //
    println!("COMPOUND TYPES");

    // group multiple values into one type
    // two primitive compound types: typles and arrays
    //
    // tuple type
    // group numbers with a variety of types
    // tuple jas fixed length - can't grow or shrink
    // create with a comma seperated list of values in parenthesis
    // each item has a type, define types in declaration
    //
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // variable binds to entire tuple... to get individual values out, we use pattern matching to
    // destructure tuple value
    //
    let (x, y, z) = tup;
    println!("Value of y is {y}");

    // can also access directly with period followed by value index
    // e.g.:
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // first index is zero
    //
    // tuple without any values is called a unit.
    // value and type written (). represents empty value or empty return type. implicitly returned
    // if nothing else is specified for expression
    //
    // ARRAY TYPE
    //
    // every element of array must have same type. arrays in rust DO have a fixed length
    //
    // express as comma seperated list in square brackets
    //
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // ^ type written with square brackets, type of elements, semicolon, number of elements
    // can initialise to inclde same value for each element like:
    let a = [3; 5]; // [3,3,3,3,3]

    // to access array elements
    // use square brackets with index. array is chunk of memory allocated on stack
    // so we can index stack address
    let first = a[0];
    let second = a[1];
    // if using an index out of bounds, program will panic and exit, if defined at run time.
    // if predefined index, compiler will catch
    // unlike low level languages like c, invalid memory cannot be accessed in rust! whilst still
    // allowing granularity of memory addressing
}
