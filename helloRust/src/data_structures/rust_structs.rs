Rust Structs
Structs
A struct (short for "structure") is a custom data structure that lets you group related values together.

You can think of a struct like a mini-database for one thing, like a person with a name and age.

Create a Struct
You define a struct using the struct keyword and place the fields (variables) inside:

Example
struct Person {
  name: String,
  age: u32,
  can_vote: bool,
}
Once you have a struct, you can create an object of it.

Then, you can access the fields of the struct using dot syntax (.):

Example
// Create a Struct called Person
struct Person {
  name: String,
  age: u32,
  can_vote: bool,
}

// Create a Person object
let user = Person {
  name: String::from("John"),
  age: 35,
  can_vote: true,
};

// Access and print the values
println!("Name: {}", user.name);
println!("Age: {}", user.age);
println!("Can vote? {}", user.can_vote);
Fields are similar to variables, but they belong to a struct. Since they are part of a larger structure (like Person or Car), they are called fields in Rust, not regular variables.

Change a Field
To change a value inside a struct, you must make the struct object mutable by using mut:

Example
struct Person {
  name: String,
  age: u32,
}

let mut user = Person {
  name: String::from("John"),
  age: 35,
};

user.age = 36; // Change value of age
println!("Name: {}", user.name);
println!("Updated age: {}", user.age);


what is Structs
struct stands for structure
is a custom data strucutre that lets you group related values together

like a person name age gender 


struct Person {
    name: String,
    age: i32,
    can_vote: bool,
}