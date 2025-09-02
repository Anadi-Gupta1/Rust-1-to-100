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


🔹 Example 1: Returning multiple values (sum and difference)
fn math_ops(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)  // return tuple
}

fn main() {
    let result = math_ops(10, 5);
    println!("Sum: {}, Difference: {}", result.0, result.1);
}


👉 Output:

Sum: 15, Difference: 5

🔹 Example 2: Destructuring the tuple
fn get_coordinates() -> (i32, i32) {
    (10, 20)
}

fn main() {
    let (x, y) = get_coordinates(); // unpack tuple into variables
    println!("X: {}, Y: {}", x, y);
}


👉 Output:

X: 10, Y: 20

🔹 Example 3: Mixing different data types
fn get_student() -> (&'static str, i32, f32) {
    ("Alice", 19, 89.5) // name, age, grade
}

fn main() {
    let student = get_student();
    println!("Name: {}, Age: {}, Grade: {}", student.0, student.1, student.2);
}


👉 Output:

Name: Alice, Age: 19, Grade: 89.5

🔹 Example 4: Tuple inside a tuple
fn get_person() -> (String, (i32, i32)) {
    (String::from("Bob"), (1999, 2025)) // name, (birth year, current year)
}

fn main() {
    let person = get_person();
    println!("Name: {}", person.0);
    println!("Birth Year: {}, Current Year: {}", (person.1).0, (person.1).1);
}


👉 Output:

Name: Bob
Birth Year: 1999, Current Year: 2025

🔹 Example 5: Empty tuple (())

Rust also has a unit tuple () which means “nothing”.

fn do_nothing() -> () {
    ()
}

fn main() {
    let result = do_nothing();
    println!("Result: {:?}", result); // prints ()
}


👉 Output:

Result: ()


✨ Summary:

Use tuples when you want to return multiple values from a function.

Access with .0, .1, .2 OR unpack them with let (a, b) = tuple.

Tuples can hold different types.

Do you want me to also make you a practice exercise sheet (small tasks) with tuples so you can try coding them yourself?

You said:
fn math_ops(a: i32, b: i32) -> (i32, i32) {

This line means a is a number and b is a number and their ouptut is also a number ?
 (String::from("Bob"), (1999, 2025)) // name, (birth year, current year)
}
cannot understand above thing

ChatGPT said:

Great question 👍 You’re catching an important detail about function signatures and tuples in Rust. Let me break this down super simple 👇

Example 1: fn math_ops(a: i32, b: i32) -> (i32, i32)

a: i32 → means a is a number (32-bit integer).

b: i32 → means b is also a number (32-bit integer).

-> (i32, i32) → means the function will return a tuple with two numbers.

So this function takes two numbers in, and gives two numbers back.
Example:

fn math_ops(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b) // tuple of two numbers
}

fn main() {
    let result = math_ops(10, 5);
    println!("Sum: {}, Difference: {}", result.0, result.1);
}


Output:

Sum: 15, Difference: 5

Example 2: (String::from("Bob"), (1999, 2025))

This looks more confusing, but it’s just a tuple inside a tuple.

First value: String::from("Bob") → a String with the name "Bob".

Second value: (1999, 2025) → another tuple with two numbers (birth year, current year).

So the whole thing is:

("Bob", (1999, 2025))


That means:

Index .0 → "Bob"

Index .1 → (1999, 2025)

Inside .1,

.0 → 1999

.1 → 2025

Example:

fn get_person() -> (String, (i32, i32)) {
    (String::from("Bob"), (1999, 2025))
}

fn main() {
    let person = get_person();
    println!("Name: {}", person.0);             // "Bob"
    println!("Birth year: {}", (person.1).0);   // 1999
    println!("Current year: {}", (person.1).1); // 2025
}


Output:

Name: Bob
Birth year: 1999
Current year: 2025


✅ So in simple words:

(i32, i32) → a tuple with 2 numbers.

(String, (i32, i32)) → a tuple where:

First part = a name (String)

Second part = another tuple with 2 numbers (years).

Would you like me to draw a diagram (boxes and arrows) to visually show how ("Bob", (1999, 2025)) is stored and accessed? That might make it crystal clear.



Example 1: Returning multiple values (sum and difference)

fn math_ops(a:i32, b:i32) -> (i32,i32) { (a+b, a-b)}
fn main (){
    let result = math_ops(10,5);
    println!("Sum: {}, Difference: {}", result.0, result.1);
}
