/*Rust Data Structures
Data Structures
In Rust, data structures are used to store and organize values.

Rust provides many built-in data structures. Each is used to handle data in different ways.

Some of the most common are:

Array
Vector (Vec)
Tuple
HashMap
We will explore all of them in detail later, but for now, here's a quick introduction to each one.

Arrays
An array in Rust is a fixed-size list of values, all of the same type.

You cannot grow or shrink an array after it's created.

To access an array element, refer to its index number.

Array indexes start with 0: [0] is the first element, [1] is the second element, etc.

Example
let fruits = ["apple", "banana", "orange"];
println!("Last fruit: {}", fruits[2]);
Vectors
A vector is a resizable array. Unlike regular arrays, vectors can grow or shrink in size.

Example
let mut fruits = vec!["apple", "banana"];
fruits.push("cherry");

println!("Last fruit: {}", fruits[2]);
Tuples
A tuple can hold multiple values of different types. It is useful when grouping different types together.

You access tuple elements using a dot and an index number, like person.1, etc:

Example
let person = ("John", 30, true);
println!("Name: {}", person.0);
println!("Age: {}", person.1);
println!("Is active: {}", person.2);
HashMaps
A HashMap stores key-value pairs. It lets you look up a value using a key.

To use HashMap, you must import it from the standard library.

Example
// Import HashMap
use std::collections::HashMap;

fn main() {
  let mut capitalCities = HashMap::new();
  capitalCities.insert("France", "Paris");
  capitalCities.insert("Japan", "Tokyo");

  println!("Capital of Japan is {}", capitalCities["Japan"]);
}
Data Structures Overview
Data Structure	Use Case	Can Grow?
Array	Fixed-size list of same-type values	No
Vector (Vec)	Growable list of same-type values	Yes
Tuple	Group different types together	No
HashMap	Key-value lookup	Yes
Next, let's take a closer look at each data structure in more detail.
*/

fn main(){
    let fruits =["apples", "bannas","cherries"];
    println!("last fruits, {}",fruits[2]);
}



