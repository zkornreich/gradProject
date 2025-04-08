#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     if let Coin::Quarter(state) = coin {
//         if state.existed_in(1900) {
//             Some(format!("{state:?} is pretty old, for America!"))
//         } else {
//             Some(format!("{state:?} is relatively new."))
//         }
//     } else {
//         None
//     }
// }

    /*The let-else syntax takes a pattern on the left side and an 
    expression on the right, (like if let), but it does not have 
    an if branch, only an else branch. If the pattern matches, it will 
    bind the value from the pattern in the outer scope. If the pattern 
    does not match, the program will flow into the else arm, 
    which must return from the function. */

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let config_max = Some(3u8);

    // Annoying to add _ exhaustive second case for discard
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // Instead: ('If-let = ' checks for pattern, 'If' checks for boolean)
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    /*Choosing between match and if let depends on whether gaining 
    conciseness is an worth losing exhaustive checking. 
    
    think of if let as a match that runs code when the value 
    matches one pattern and then ignores all other values.
    
    Else with an if let is the same as the block of code that would 
    go with the '_' case in the match expression that is equivalent 
    to the if let and else. 
    
    i.e. Quarter variant also held a UsState value. Counting all 
    non-quarter coins we see while also announcing the state of 
    the quarters: match expression vs if-let*/

    let mut countMatch = 0;
    let mut countIfLet = 0;

    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
    
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

}
