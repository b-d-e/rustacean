struct User {
    // Struct is similar to a tuple (multiple items, different types allowed), but
    // different in that each peice of data is explicitly named
    // => don't have to rely on item order to access
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64, // note trailing ','
}

fn main() {
    // to use a struct, we create an instance of it, specifying values for fields
    let mut user1 = User {
        // make sure to make mutable if wanting to change values! n.b. WHOLE
        // instance must be mutable, not item
        active: true,
        username: String::from("user123"),
        email: String::from("a@b.com"),
        sign_in_count: 6,
    };

    user1.email = String::from("c@d.com");

    // generate instances from other instances - use Struct Update
    let user2 = User {
        email: String::from("newemail@domain.co"), // changed values MUST come first
        ..user1                                    // take same values if not specified here
    }; // far more elegant than doing, e.g., user1.active, user1.username, etc

    // assingment-like '=', meaning data is moved - i.e. user1 as a whole can no longer be used, as
    // String type is borrowed NOT copied.
    // if just retaining active and sign_in_count, we would still be able to use user1, as bools
    // and u64 implement the copy trait (not stored on heap)

    // calls of other things
    build_user("user@name.com".to_string(), "username".to_string());
    better_build_user("user@name.com".to_string(), "username".to_string());

    main2();

    unitstruct()
}

// un-rusty way to build through function
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, // tedious to repeat parameter names if same
        email: email,
        sign_in_count: 1,
    }
}

// it is better to use the Field init shorthand
fn better_build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // implicitly uses parameter value here,l as long as the names are identical
        email,
        sign_in_count: 1,
    }
}

// TUPLE STRUCTS
// these are structs that look similar to tuples
// benefit from added meaning of Struct name, but don't have names associated with their fields,
// just types
// useful when naming individual fields is redundant or unecessarily verbose
// start with struct keyword and name then types, e.g.:
struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

fn main2() {
    let black = Colour(0, 0, 0); // we're assuming RGB is implicit in the fact it is a colour
    let origin = Point(0, 0, 0); // similarly, some X,Y,Z precedence is assumed

    // so why use structs at all - for the names. e.g., can define function that will only accept
    // Colour, so can't accidentally pass a Point in
    // each struct is it's own type, even if consituent fields are identically typed
}

// Unit-Like Structs
// these have no fields, similar to (), but are named
// useful to implement trait on type, but without any data to store in the type (chapter 10 covers
// this more)
// e.g.

struct AlwaysEqual;

fn unitstruct() {
    let subject = AlwaysEqual;
    // no need for brackets
    // down the line, we can implement a trait such that every instance is always equal to every
    // other instance, even without data
}

// Struct data ownership
// in User, we use owned String rather than &str slice type
// deliberate - we want every instance to own all its data, and data to be as long lived as the
// struct itself
//
// structs can also store references to data owned by something else, but this requires lifetimes
// to be used - see chapter 10
// for instance, the below will currently fail to compile:

struct User2 {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main3() {
    let user1 = User2 {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
// we get '    |               ^ expected named lifetime parameter'
// for now, to avoid this use Owened types like String rather than references like &str
