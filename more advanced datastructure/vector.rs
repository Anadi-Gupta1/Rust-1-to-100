// Vectors in Rust allow you to store more than one value in a single data structure.
// All the values are stored next to each other in memory, making access fast and efficient.
// Vectors are growable arrays, meaning you can add or remove elements dynamically.

fn main() {
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
}// Vectors in Rust allow you to store more than one value in a single data structure.
// All the values are stored next to each other in memory, making access fast and efficient.
// Vectors are growable arrays, meaning you can add or remove elements dynamically.

fn main() {
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