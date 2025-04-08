fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    // Providing a reference maintains s1 as the owner of the string object
    // the memory reference is an immutable borrow
    change(&mut s1);
    s1.push_str("!!");

    println!("The length of '{s1}' was {len}.");

    let mut s = String::from("borrowing?");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}