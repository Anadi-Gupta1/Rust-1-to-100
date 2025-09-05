what is generics 
Generics is a feature in programming languages that allows you to write flexible and reusable code by defining functions, types, or data structures that can operate on different data types without being tied to a specific one. In languages like Rust, generics enable you to create components that can work with any data type while maintaining type safety.
Generics in Rust
Generics in Rust allow you to write functions, structs, enums, and traits that can operate on different data types without sacrificing type safety. This is achieved by using type parameters, which are placeholders for actual types that will be specified when the generic item is used.
Defining Generic Functions
You can define a generic function by using angle brackets <> to specify type parameters. Here's an example
fn main() {
    // A generic function that takes two parameters of the same type and returns the larger one
    fn largest<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
    }

    let int1 = 10;
    let int2 = 20;
    let result = largest(int1, int2);
    println!("The largest integer is: {}", result);

    let float1 = 10.5;
    let float2 = 20.3;
    let result = largest(float1, float2);
    println!("The largest float is: {}", result);
}
In this example, the function largest is defined with a type parameter T that must implement the PartialOrd trait, which allows comparison using the > operator. The function can then be used with different types, such as integers and floating-point numbers.
Defining Generic Structs
You can also define structs with generic type parameters. Here's an example:
struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 2.5 };

    println!("Integer Point: ({}, {})", int_point.x, int_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
}
In this example, the struct Point is defined with a type parameter T, allowing it to store coordinates of any type.
Defining Generic Enums
You can also define enums with generic type parameters. Here's an example:
enum Option<T> {
    Some(T),
    None,
}
fn main() {
    let some_number = Option::Some(5);
    let some_string = Option::Some("Hello");
    let no_value: Option<i32> = Option::None;

    match some_number {
        Option::Some(value) => println!("Got a number: {}", value),
        Option::None => println!("No value"),
    }

    match some_string {
        Option::Some(value) => println!("Got a string: {}", value),
        Option::None => println!("No value"),
    }

    match no_value {
        Option::Some(value) => println!("Got a number: {}", value),
        Option::None => println!("No value"),
    }
}
    // Creating a new, empty vector of i32 (32-bit integers)
    let mut vec: Vec<i32> = Vec::new();

    // You can also create a vector with initial values using the vec! macro
    let mut vec2 = vec![1, 2, 3];

    // Adding elements to a vector using push
    vec.push(10);
    vec.push(20);
    vec.push(30);

    // Accessing elements by index (indexing starts at 0)
    println!("First element: {}", vec[0]);

    // Safe access using get (returns Option<&T>)
    match vec.get(2) {
        Some(value) => println!("Third element: {}", value),
        None => println!("No third element found"),
    }

    // Iterating over a vector
    for val in &vec {
        println!("Value: {}", val);
    }

    // Modifying elements via mutable reference
    for val in &mut vec {
        *val += 1;
    }
    println!("After incrementing: {:?}", vec);

    // Removing the last element
    vec.pop();
    println!("After pop: {:?}", vec);

    // Vectors can store any type, including custom structs
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let mut points: Vec<Point> = Vec::new();
    points.push(Point { x: 1, y: 2 });
    points.push(Point { x: 3, y: 4 });
    println!("Points: {:?}", points);

    // Summary:
    // - Vec<T> is the type for a vector storing elements of type T
    // - Use push to add, pop to remove last, get for safe access
    // - Vectors are heap-allocated and can grow/shrink dynamically
}
    // A generic function that takes two parameters of the same type and returns the larger one
    fn largest<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
    }

    let int1 = 10;
    let int2 = 20;
    let result = largest(int1, int2);
    println!("The largest integer is: {}", result);

    let float1 = 10.5;
    let float2 = 20.3;
    let result = largest(float1, float2);
    println!("The largest float is: {}", result);
}
fn main() {
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 2.5 };

    println!("Integer Point: ({}, {})", int_point.x, int_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
}