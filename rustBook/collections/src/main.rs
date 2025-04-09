// Unlike the built-in array and tuple types, the data these 
//  collections point to is stored on the heap, Thus, the amount 
//  of data does not need to be known at compile time and 
//  can grow or shrink as the program runs.

// Rust needs to know what types will be in the vector at compile time 

// Vector - store a variable number of values next to each other.
// String is a collection of characters.
// Hash map allows you to associate a value with a specific key. 

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Declaring a vector
    let _v_e: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    let read: Option<&i32> = v.get(3);
    match read {
        Some(read) => println!("The element is {read}\t |"),
        None => print!("There is no element at the specified location.\t | "),
    }

    v.pop();
    let read: Option<&i32> = v.get(3);
    match read {
        Some(read) => println!("The element is {read}"),
        None => println!("There is no element at the specified location."),
    }

    // let does_not_exist = &v[100]; --Panic due to index out of bounds
    // let does_not_exist = v.get(100); assigns None

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    
    // Then we can create a vector to hold that enum and so, 
    //  ultimately, hold different types.
    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for element in &mut row{
        match element {
            SpreadsheetCell::Int(x) => *x+=10,
            SpreadsheetCell::Text(s) => print!("Text! {:?}\t | ", s),
            SpreadsheetCell::Float(y) => *y*=2.0,
        }
    }

    println!("{:?}", row);

    // If you don’t know the exhaustive set of types a program will 
    // get at runtime to store in a vector, the enum technique fails.
    // Instead, you can use a trait object (Later)

    // STRINGS
    // let mut s = String::new();
    let data = "foo"; //String SLICE
    let mut s = data.to_string(); //Makes string type
    let _s_too = "initial contents".to_string(); //Also works!
    let exclaim = String::from("!");

    s.push_str("bar");
    s = s + &exclaim;
    s.push_str(&exclaim);
    s.push('!');

    print!("{s}\t | ");

    let s1 = String::from("Hello, ");
    let s2 = "world!".to_string();
    let s1 = s1 + &s2;
    // note s1 has been moved to s3 here and can no longer be used
    // S1 has to be String, not str or &s

    print!("{s1}\t | ");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    // format! DOES NOT TAKE ANY OWNERSHIP
    println!("{}", s);

    print!("{:?}\n", s.chars());
    println!("{:?}", s.bytes());
    
    match s.find('o') {
        Some(c) => print!("{c} \t"),
        _ => (),
    };

    match s.chars().nth(5) {
        Some(c) => println!("{c}"),
        _ => (),
    };

    // HASH MAPS (Dict)
    // type HashMap<K, V> stores a mapping of keys of type K to 
    // values of type V using a hashing function, which determines 
    // how it places these keys and values into memory

    use std::collections::HashMap;
    // Least commonly used, so not a part of implied prefix
    // Must "use" std:collections to bring into scope

    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 40);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get method returns an Option<&V>; 
    // if there’s no value for that key, get will return None. 
    // Calls copied to get an Option<i32> rather than an Option<&i32>, 
    // unwrap_or(0) to set score to zero if scores doesn’t have an entry 
    //      for the key.

    let _scr = scores.get(&team_name).unwrap_or(&0);
    // Can avoid using .copied(), make unwarp_or(&0)

    print!("{score}\t"); // score and scr are equivalent

    for (key, value) in &scores {
        print!("{key}: {value} | ");
    }

    // types that implement copy (ints) are copied into hashmap
    // Others, like Strings, are moved and ownership is transferred to HM
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    //      Ownership has transferred to the hashmap

    // handle the case when a key already has a value assigned.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    print!("\n{scores:?} | ");
    // Insert - Replace the old value with the new value 
    //      disregarding the old value. 

    // check whether the key for the Yellow/blue team has a value.
    // If it doesn’t, we want to insert the value 50
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?} | ");

    // Entry - Keep the old value and ignore the new value, adding the new 
    //      value only if the key doesn’t already have a value.

    // Combine the old value and the new value.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // split_whitespace method returns an iterator over subslices, 
    //      separated by whitespace, of the value in text. 
    // The or_insert method returns a mutable reference (&mut V) to the 
    //   value for the specified key. Mutable reference is stored in the 
    //   count variable, so in order to assign to that value, we must 
    //   first dereference count using the asterisk (*). 
    // The mutable reference goes out of scope at the end of the for loop
    println!("{map:?}");

}