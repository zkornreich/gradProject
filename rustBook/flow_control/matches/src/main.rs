enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    let c = Coin::Nickel;
    println!("{:#?}", value_in_cents(c));
    println!("{:#?}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         None => None,
    //         Some(i) => Some(i + 1),
    //     }
    // }

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), //Catches all other u8s and binds to other
        //_ => (); 
            // use _ if not planning to use the matched variable
            // () is the empty tuple (unit value --> NOP)
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}
