// String - dynamic (mut) heap string type - variable size and assignment
// str - immutable borrow of char sequence - 
//      dynamic length & stored anywhere

// String
//      let my_string = String::from("hello world");
// String Literal
//      let my_string_literal = "hello world";

fn main () {
    let s = String::from("hello world");

    // 0-5
    let _hello = &s[0..5];
    let _hello1 = &s[..5];

    // 6-11
    let _world = &s[6..];
    let _world1 = &s[6..11];

    // Whole word
    let _slice = &s[0..s.len()];
    let _slice1 = &s[..];

    let word = first_word(&s[..]);
    // s.clear(); // error! 

    // s.clear takes a mutable borrow, so all immutable borrows 
    //     should be dropped 
    // Immutable borrow value used below in println!
    //     This immutable borrow must be dropped when the new mutable
    //     borrow was made. Thus referencing it below causes panic

    println!("the first word is: {word}"); 

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let w_lit = "Literal String";
    let w = first_word(w_lit);
    println!("{w}");
    
    // General Slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("{:?}", slice); //Need format specifier for printing arr
}
// First word as index:

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// First word as slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}