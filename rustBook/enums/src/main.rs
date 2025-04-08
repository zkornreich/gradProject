// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

/* put any kind of data inside an enum variant: 
strings, numeric types, or structs, for example. 
You can even include another enum!*/

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,                       //No Data
    Move { x: i32, y: i32 },    //Named fields, like a struct
    Write(String),              //One string
    ChangeColor(i32, i32, i32), //Three i32s
}

impl Message {
    fn call(&self) {
        println!("{:#?}", &self)
    }
}

#[allow(unused)]
fn main (){
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: six,
    //     address: String::from("::1"),
    // };
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);
    
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    optionStuff();
}

// fn route(ip_kind: IpAddrKind) {}

/* NOTES ON OPTION & NO NULL IN RUST
-- Null is a value that is currently invalid or absent for some reason
-- Important concept, but null as a type is bad implementation
-- Need to be able to know if value is non existent, 
    but having a null type makes more issues than solves

--> OPTION ENUM:
Rust does not have nulls, but it does have an enum 
that can encode the concept of a value being present or absent.

enum Option<T> {
    None,
    Some(T),
}

- Included in the prelude; don’t need to bring it into scope explicitly. 
- Can use Some and None directly without the Option:: prefix. 
-- The Option<T> enum is just a regular enum
--- Some(T) and None are just variants of type Option<T>.
*/

fn optionStuff (){
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

/*
When we have a None value, it means the same thing as null: 
we don’t have a valid value. 

Option<T> and T (where T can be any type) are different types, 
the compiler won’t let us use an Option<T> value as if it were 
definitely a valid value. For example, this code won’t compile, 
because it’s trying to add an i8 to an Option<i8>: 
*/

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}

/*
Rust doesn’t understand how to add an i8 and an Option<i8>, because they’re different types. 
When we have a value of a type like i8 in Rust, the compiler will ensure that we always have a valid value. 
Only when we have an Option<i8> (or whatever type of value we’re working with) do we have to worry about possibly not having a value, 
and the compiler will make sure we handle that case before using the value.

In other words, you have to convert an Option<T> to a T before you can perform T operations with it. 
Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

Eliminating the risk of incorrectly assuming a not-null value helps you to be more confident in your code. In order to have a value that can possibly be null, 
you must explicitly opt in by making the type of that value Option<T>. Then, when you use that value, you are required to explicitly handle the case when the value is null. 
Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null. 

This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.

So how do you get the T value out of a Some variant when you have a value of type Option<T> so that you can use that value? 
The Option<T> enum has a large number of methods that are useful in a variety of situations; you can check them out in its documentation. 
Becoming familiar with the methods on Option<T> will be extremely useful in your journey with Rust.

In general, in order to use an Option<T> value, you want to have code that will handle each variant. 
You want some code that will run only when you have a Some(T) value, and this code is allowed to use the inner T. 
You want some other code to run only if you have a None value, and that code doesn’t have a T value available. 

The match expression is a control flow construct that does just this when used with enums: 
it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value. */