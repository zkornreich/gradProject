//Rust doesn’t allow us to mark only certain fields as mutable
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
//  Point and color are NOT interchangable despite yielding 
//  identical tuples

fn main() {
    let mut user1 = build_user(String::from("user@user.com"), String::from("username"));
    user1.email = String::from("newUser@user.com");
    user1.active = true;
    println!("{:?} | {:?}", user1.username, user1.sign_in_count);

    println!("{:?}", user1);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{:?}", user2);
    //println!("{:?}", user1); //Won't work because elements of user1 are being borrowed

    let black = Color(0, 0, 0);
    //let origin = Point(0, 0, 0);

    println!("{:?} | {:?} | {:?}", black.0, black.1, black.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        sign_in_count: 1,
        
        //Not need set because variable and struct names are the same
        username,
        email,
    }
}

// Struct must be String instead of &str so that the struct owns all of it's data.
//     Using &str will panic unless lifetimes are specified