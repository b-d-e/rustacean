fn main() {
    // match allows you to compare a value against a series of atterns and execute code based on
    // which pattern matches
    // patterns can be made of literal values, variable names, wild cards, and more
    println!("{}", value_in_cents(Coin::Penny));
    value_in_cents_2(Coin_2::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // each 'arm' has two parts - pattern and code to return
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// patterns that bind to values
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin_2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_2(coin: Coin_2) -> u8 {
    match coin {
        Coin_2::Penny => 1,
        Coin_2::Nickel => 5,
        Coin_2::Dime => 10,
        Coin_2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// matches must be exhaustive - the arms patterns have to cover all possibilities
// compiler will complain if you don't check them all
// this especially useful with Option<T> where Rust forces us to consider what to do if None

// catch all patterns
fn catch_all() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

// similarly, if we want to catch all but don't want to use the value, we can use a wildcard:
fn wildcard() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}
