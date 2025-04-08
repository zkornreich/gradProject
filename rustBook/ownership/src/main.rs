/*
let s1 = String::from("hello");
- makes S1 an object on the stack that references the 
-   memory location on the heap that contains "Hello"
    let s2 = s1;
- Makes s2 a reference equivalent to s1 (stack object
-   pointing to the heap). S2 is the new owner of that 
-   heap memory and s1 is dropped.

    println!("{s1}, world!");
- Panics because s1 is invalid due to ownership transfer to s2.

C would allow both pointers to read, but seg fault upon write

to create multiple heap allocations, use 
let s2 = s1.clone();

*/

fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    //takes_ownership(s.clone());   // A clone of s is made and given to the function

                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    //let d = s;                    // fails unless a clone of s is passed in above

    makes_copy(x);                  // because i32 implements the Copy trait,
                                    // x does NOT move into the function,
//    println!("{} | {d}", x);              // so it's okay to use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

/*
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
    
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
*/