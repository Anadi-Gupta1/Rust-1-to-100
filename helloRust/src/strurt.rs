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