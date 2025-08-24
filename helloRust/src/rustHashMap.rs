Rust HashMap
HashMap
A HashMap is a collection of key/value pairs.

HashMaps are great when you want to store values and find them by a key.

To use HashMap, you must import it from Rust's standard library:

use std::collections::HashMap;
Create a HashMap
You can create a new, empty HashMap and add items to it:

Example
// Import HashMap
use std::collections::HashMap;

fn main() {
  // Create a HashMap called capitalCities
  let mut capitalCities = HashMap::new();

  // Add keys and values (Country, City)
  capitalCities.insert("England", "London");
  capitalCities.insert("Germany", "Berlin");
  capitalCities.insert("Norway", "Oslo");

  println!("{:?}", capitalCities);
}
Access Values
You can use the .get() method to access a value in a HashMap by its key:

Example
let mut capitalCities = HashMap::new();

capitalCities.insert("England", "London");
capitalCities.insert("Germany", "Berlin");
capitalCities.insert("Norway", "Oslo");

if let Some(city) = capitalCities.get("England") {
  println!("The capital of England is {}.", city);
} else {
  println!("England is not in the map.");
}
Update Values
If you insert a new value using a key that already exists, the old value is replaced with the new one:

Example
let mut capitalCities = HashMap::new();

capitalCities.insert("England", "London");
capitalCities.insert("England", "Berlin");

println!("{:?}", capitalCities);
Remove Values
To remove a key from a HashMap, use the .remove() method:

Example
let mut capitalCities = HashMap::new();

// Add keys and values (Country, City)
capitalCities.insert("England", "London");
capitalCities.insert("Germany", "Berlin");
capitalCities.insert("Norway", "Oslo");

// Remove the key "England"
capitalCities.remove("England");

println!("{:?}", capitalCities);
Loop Through a HashMap
You can use a for loop to go through all key/value pairs:

Example
let mut capitalCities = HashMap::new();

// Add keys and values (Country, City)
capitalCities.insert("England", "London");
capitalCities.insert("Germany", "Berlin");
capitalCities.insert("Norway", "Oslo");

// Loop through the HashMap
for (country, city) in &capitalCities {
  println!("The capital of {} is {}.", country, city);
}



use std ::collections::HashMap;

fn main(){
    let mut moneyHeistCities = HashMap::new();
    moneyHeistCities.insert("Tokyo", "Japan");
    moneyHeistCities.insert("Berlin", "Germany");
    moneyHeistCities.insert("Nairobi", "Kenya");


    println!("{:?}", moneyHeistCities);
}



Access Values
You can use the .get() method to access a value in a HashMap by its key:


use std::collections::HashMap;
fn main() {
    let mut moneyHeistCities = HashMap::new();
    moneyHeistCities.insert("Tokyo", "Japan");
    moneyHeistCities.insert("Berlin", "Germany");
    moneyHeistCities.insert("Nairobi", "Kenya");

    if let Some(city) = moneyHeistCities.get("Tokyo") {
        println!("The capital of Tokyo is {}.", city);
    } else {
        println!("Tokyo is not in the map.");
    }
}