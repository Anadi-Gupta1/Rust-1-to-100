use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// Enums and Pattern Matching
#[derive(Debug, Clone)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

// Structs and Implementations
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    skills: Vec<String>,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age,
            skills: Vec::new(),
        }
    }

    fn add_skill(&mut self, skill: String) {
        self.skills.push(skill);
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

// Traits
trait Area {
    fn area(&self) -> f64;
}

impl Area for Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

// Generic Functions
fn find_max<T: PartialOrd + Clone>(list: &[T]) -> Option<T> {
    list.iter().max().cloned()
}

// Iterator Patterns and Functional Programming
fn demonstrate_iterators() {
    println!("\n=== Iterator Demonstrations ===");
    
    let numbers: Vec<i32> = (1..=100).collect();
    
    // Basic iteration
    println!("First 10 numbers:");
    numbers.iter().take(10).for_each(|n| print!("{} ", n));
    println!();
    
    // Filter and map
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .take(5)
        .collect();
    println!("First 5 even squares: {:?}", even_squares);
    
    // Reduce operations
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().take(5).product();
    println!("Sum of 1-100: {}, Product of 1-5: {}", sum, product);
    
    // Group by using fold
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
        .iter()
        .take(20)
        .partition(|&&x| x % 2 == 0);
    println!("Evens: {:?}", evens);
    println!("Odds: {:?}", odds);
}

// Advanced Pattern Matching
fn advanced_pattern_matching() {
    println!("\n=== Advanced Pattern Matching ===");
    
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    
    for shape in &shapes {
        match shape {
            Shape::Circle(r) if *r > 3.0 => println!("Large circle with area: {:.2}", shape.area()),
            Shape::Circle(r) => println!("Small circle with radius: {}", r),
            Shape::Rectangle(w, h) if w == h => println!("Square with area: {:.2}", shape.area()),
            Shape::Rectangle(w, h) => println!("Rectangle {}x{} with area: {:.2}", w, h, shape.area()),
            Shape::Triangle(a, b, c) => {
                if a == b && b == c {
                    println!("Equilateral triangle with area: {:.2}", shape.area());
                } else {
                    println!("Triangle with sides {}, {}, {} and area: {:.2}", a, b, c, shape.area());
                }
            }
        }
    }
}

// Error Handling with Result and Option
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn demonstrate_error_handling() {
    println!("\n=== Error Handling ===");
    
    let operations = vec![(10.0, 2.0), (15.0, 3.0), (8.0, 0.0), (20.0, 4.0)];
    
    for (a, b) in operations {
        match safe_divide(a, b) {
            Ok(result) => println!("{} / {} = {:.2}", a, b, result),
            Err(e) => println!("Error dividing {} by {}: {}", a, b, e),
        }
    }
    
    // Option handling
    let numbers = vec![1, 2, 3, 4, 5];
    let index = 10;
    
    match numbers.get(index) {
        Some(value) => println!("Value at index {}: {}", index, value),
        None => println!("Index {} is out of bounds", index),
    }
}

// Closures and Higher-Order Functions
fn demonstrate_closures() {
    println!("\n=== Closures and Higher-Order Functions ===");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Closure capturing environment
    let threshold = 5;
    let above_threshold: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x > threshold)
        .cloned()
        .collect();
    println!("Numbers above {}: {:?}", threshold, above_threshold);
    
    // Different closure types
    let add_one = |x: i32| x + 1;
    let multiply = |x: i32, y: i32| x * y;
    
    let transformed: Vec<i32> = numbers
        .iter()
        .map(|&x| add_one(x))
        .map(|x| multiply(x, 2))
        .collect();
    println!("Transformed numbers: {:?}", transformed);
    
    // Higher-order function
    fn apply_operation<F>(numbers: &[i32], op: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32,
    {
        numbers.iter().map(|&x| op(x)).collect()
    }
    
    let squared = apply_operation(&numbers, |x| x * x);
    println!("Squared numbers: {:?}", squared);
}

// Collections and Data Structures
fn demonstrate_collections() {
    println!("\n=== Collections and Data Structures ===");
    
    // HashMap
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    
    println!("Scores:");
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
    
    // Complex data structures
    let mut people = Vec::new();
    let mut alice = Person::new("Alice".to_string(), 25);
    alice.add_skill("Rust".to_string());
    alice.add_skill("Python".to_string());
    
    let mut bob = Person::new("Bob".to_string(), 17);
    bob.add_skill("JavaScript".to_string());
    
    people.push(alice);
    people.push(bob);
    
    println!("\nPeople:");
    for person in &people {
        println!("{:?} - Adult: {}", person, person.is_adult());
    }
    
    // Find adults with specific skills
    let rust_adults: Vec<&Person> = people
        .iter()
        .filter(|person| person.is_adult() && person.skills.contains(&"Rust".to_string()))
        .collect();
    
    println!("\nRust developers who are adults:");
    for person in rust_adults {
        println!("- {}", person.name);
    }
}

// Multithreading and Concurrency
fn demonstrate_concurrency() {
    println!("\n=== Concurrency and Threading ===");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += i;
            println!("Thread {} updated counter", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *counter.lock().unwrap());
}

// Lifetime Management
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn demonstrate_lifetimes() {
    println!("\n=== Lifetime Management ===");
    
    let string1 = "Hello World";
    let string2 = "Rust Programming";
    
    let result = longest(string1, string2);
    println!("Longest string: '{}'", result);
}

// Macros
macro_rules! create_function {
    ($name:ident, $op:tt) => {
        fn $name(a: i32, b: i32) -> i32 {
            a $op b
        }
    };
}

create_function!(add, +);
create_function!(multiply, *);
create_function!(subtract, -);

fn demonstrate_macros() {
    println!("\n=== Macros ===");
    println!("5 + 3 = {}", add(5, 3));
    println!("5 * 3 = {}", multiply(5, 3));
    println!("5 - 3 = {}", subtract(5, 3));
}

fn main() {
    println!("🦀 Advanced Rust Programming Examples 🦀");
    println!("==========================================");
    
    // Basic confirmation
    println!("✅ Laptop is working and ready for advanced Rust!");
    println!("📅 Today's date: September 2, 2025");
    println!("🎯 Learning Rust with comprehensive examples!");
    println!("👤 Author: Anadi Gupta");
    println!("📧 Contact: anadigupta5555@gmail.com");
    println!();
    
    // Run all demonstrations
    demonstrate_iterators();
    advanced_pattern_matching();
    demonstrate_error_handling();
    demonstrate_closures();
    demonstrate_collections();
    demonstrate_lifetimes();
    demonstrate_macros();
    demonstrate_concurrency();
    
    // Final statistics
    let numbers: Vec<i32> = (1..=1000).collect();
    let stats = numbers.iter().fold((0, i32::MAX, i32::MIN), |(sum, min, max), &x| {
        (sum + x, min.min(x), max.max(x))
    });
    
    println!("\n=== Final Statistics ===");
    println!("Sum: {}, Min: {}, Max: {}", stats.0, stats.1, stats.2);
    
    // Demonstrate generic function
    let max_number = find_max(&numbers).unwrap_or(0);
    println!("Maximum number found: {}", max_number);
    
    println!("\n🎉 Advanced Rust demonstration complete!");
}