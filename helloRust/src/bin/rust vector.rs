Rust Vectors
Vectors
A vector is a resizable array. Unlike regular arrays, vectors can grow or shrink in size.

Creating a Vector
To create a vector, use the vec! macro:

Example
let fruits = vec!["apple", "banana", "orange"];
This creates a vector with three string elements.

Access Vector Elements
You can access values in a vector using index numbers (just like arrays):

Example
let fruits = vec!["apple", "banana", "orange"];
println!("First fruit: {}", fruits[0]);
Change Vector Values
To change a value in the vector, refer to the index number and assign a new value.

Remember to make the vector mutable (using the mut keyword):

Example
let mut fruits = vec!["apple", "banana", "orange"];
fruits[0] = "grape";
println!("New first fruit: {}", fruits[0]);
Add Elements to a Vector
You can add a new element to the end of a vector using the push() method:

Example
let mut fruits = vec!["apple", "banana"];
fruits.push("cherry");
println!("{:?}", fruits); // ["apple", "banana", "cherry"]
Remove Elements from a Vector
To remove the last element from a vector, use pop():

Example
let mut fruits = vec!["apple", "banana", "cherry"];
fruits.pop();
println!("{:?}", fruits); // ["apple", "banana"]
Add or Remove Elements at a Specified Index
Rust vectors are designed to grow and shrink at the end, but you can also add or remove elements at the beginning or at a specified index.

Use insert() to add an item at a specified index:

Example
Add "apple" to the beginning of the vector:

let mut fruits = vec!["banana", "orange"];
fruits.insert(0, "apple");
println!("{:?}", fruits); // ["apple", "banana", "orange"]
Example
Add "apple" in the middle of the vector:

let mut fruits = vec!["banana", "orange"];
fruits.insert(1, "apple");
println!("{:?}", fruits); // ["banana", "apple", "orange"]
Remove the First Item
Use remove() to remove an element from a specified index:

Example
Remove the first item in the vector:

let mut fruits = vec!["apple", "banana", "orange"];
fruits.remove(0);
println!("{:?}", fruits); // ["banana", "orange"]
Note: Adding or removing elements from the beginning is slower than at the end, because all the other elements have to shift positions.

Vector Length
You can find out how many elements there are in a vector using the .len() method:

Example
let fruits = vec!["apple", "banana", "cherry"];
println!("There are {} fruits.", fruits.len());
Loop Through a Vector
Just like arrays, you can use a for loop to go through all the values in a vector:

Example
let fruits = vec!["apple", "banana", "orange"];
for fruit in &fruits {
  println!("I like {}.", fruit);
}




pop last vala element hata hai koi si bhi list se
for eg


fn main() {
    let moneyheist = vec!["Tokyo", "Berlin", "Nairobi"];
    moneyheist.pop();
}

output of this program will be 
["Tokyo", "Berlin"]


now fn main (){
    let moneyheist = vec!["Tokyo", "Berlin", "Nairobi"];
    moneyheist.remove(0);
}

output of this program will be
["Berlin", "Nairobi"]


when we want to add elements in the middle of the list 
so we can proceed by 

fn main (){
    let mut moneyheist = vec!["Tokyo", "Berlin", "Nairobi"];
    moneyheist.insert(1,"Denver");
    println!("{:?}",moneyheist);
}

for particular position add some element in the list we use insert command

if i just want to add the element in the list such that
fn main (){
    let mut moneyheist = vec!["Tokyo", "Berlin", "Nairobi"];
    moneyheist.push("Denver");
    println!("{:?}",moneyheist);
}

we use the push command to add the list


to remove the first element from the list
fn main() {
    let mut moneyheist = vec!["Tokyo", "Berlin", "Nairobi"];
    moneyheist.remove(0);
    println!("{:?}",moneyheist);
}



vector Length
fn main (){
    let fruits = ["apple", "banana", "cherry"];
println!("{:?}", fruits.len());
}


Loop Through a Vector
Just like arrays, you can use a for loop to go through all the values in a vector:


fn main() {
    let moneyheist = vec!["Tokyo", "Berlin", "Nairobi"];
    for character in moneyheist {
        println!("I like {}.", character);
    }
}