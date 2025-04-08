// fn main() {
//     //const THREE_HOURS_IN_SECONDS : u32 = 60*60*3;

//     let x : u32 = 5;
//     println!("The value of x is: {x}");
//     {
//         let x = x*2;
//         println!("Inner scope value {x}");
//     }
//     let x = "hi";
//     println!("The value of x is: {x}");
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();

    let i: usize = loop {
        println!("Please enter an array index.");

        index.clear();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index was not a number");

        if index < a.len(){
            break index;
        }
        else {
            println!("invalid Index. Array max index is {}", a.len()-1);
        }
    };

    let element = a[i];
    println!("The value of the element at index {i} is: {element}");
}