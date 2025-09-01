//what is struct
    A struct in Rust is a custom data type that lets you group related data together. It allows you to create complex data types by combining multiple values of different types into a single unit. Structs are useful for modeling real-world entities and organizing data in a meaningful way.

    It basis structure the data 

    for ex in c++ ther is classes and objects the struct is similar to that

    Structs
Structs in rust let you structure data together. Similar to objects in javascript
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    print!("User 1 username: {:?}", user1.username);
}
Side quest - Learning about traits
Try running experiments around
 mutable and immutable references
Ownership transfer
for structs
Code 1 - Only stack types in the struct
struct User {
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        sign_in_count: 1,
    };

    print_name(user1);
    print!("User 1 username: {}", user1.active); // Error - can not use borrowed value
}

fn print_name(user1: User) {
    print!("User 1 username: {}", user1.active);
}

 
Add the copy trait
#[derive(Copy, Clone)]
struct User {
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        sign_in_count: 1,
    };

    print_name(user1);
    print!("User 1 username: {}", user1.active); // Error goes away because user1 is copied
}

fn print_name(user1: User) {
    print!("User 1 username: {}", user1.active);
}

 
Code 2 - Add strings 
struct User {
    active: bool,
    sign_in_count: u64,
    username: String,
}

fn main() {
    let mut user1 = User {
        active: true,
        sign_in_count: 1,
        username: "harkirat".to_string()
    };

    change_name(user1);
    print!("User 1 username: {}", user1.active); // Error - can not use borrowed value
}

fn change_name(user1: User) {
    print!("User 1 username: {:?}", user1.active);
}

 
Try adding the Copy trait (you wont be able to because strings dont implement them, use clone trait instead)
#[derive(Clone)]
struct User {
    active: bool,
    sign_in_count: u64,
    username: String,
}

fn main() {
    let mut user1 = User {
        active: true,
        sign_in_count: 1,
        username: "harkirat".to_string()
    };

    change_name(user1.clone());
    print!("User 1 username: {}", user1.active); // Error - can not use borrowed value
}

fn change_name(user1: User) {
    print!("User 1 username: {:?}", user1.active);
}