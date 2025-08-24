Rust Tuples
Tuples
A tuple is a group of values of different types, stored in a single variable.

Tuples are useful when you want to return or work with multiple values together.

Create a Tuple
Tuples are written using parentheses (), with values separated by commas:

Example
let person = ("John", 30, true);
This tuple contains a &str, an i32, and a bool.

Access Tuple Values
You can access tuple values by using a dot . followed by the index:

Example
let person = ("John", 30, true);
println!("Name: {}", person.0);
println!("Age: {}", person.1);
println!("Is active: {}", person.2);
Unpack a Tuple
When we create a tuple, we normally assign values to it. This is called "packing" a tuple:

Example
let person = ("Jenny", 45, false);
But, in Rust, we are also allowed to extract the values back into variables. This is called "unpacking":

Example
let person = ("Jenny", 45, false);
let (name, age, active) = person;

println!("Name: {}", name);
println!("Age: {}", age);
println!("Active: {}", active);
Return a Tuple from a Function
Tuples are often used to return multiple values from a function:

Example
fn get_user() -> (String, i32) {
  (String::from("Liam"), 25)
}

fn main() {
  let user = get_user();
  println!("User: {} ({} years old)", user.0, user.1);
}



But, in Rust, we are also allowed to extract the values back into variables. This is called "unpacking":

Example
let person = ("Jenny", 45, false);
let (name, age, active) = person;

println!("Name: {}", name);
println!("Age: {}", age);
println!("Active: {}", active);
