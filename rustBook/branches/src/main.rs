// use std::io;

// fn main() {
//     let mut number : String = String::new();

//     println!("Enter a number: ");

//     io::stdin().read_line(&mut number).expect("Invalid Input");
//     let number :i32 = match number.trim().parse() {
//         Ok(number) => number,
//         Err(_) => -1,
//     };

//     let val = if number == 0 {
//         println!("condition was true => {number}");
//         true
//     } else if number > 0 {
//         println!("condition was false => {number}");
//         false
//     } else {
//         println!("condition was negative => {number}");
//         false
//     };

//     println!("{val}");
// }

fn main() {
    let mut count = 0;
    let _c : u32 = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up count;
            }
            remaining -= 1;
        }

        count += 1;
    };
    println!("End count = {count}");

    for number in (1..4).rev() { // 4..0 doesn't work
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}