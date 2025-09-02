1. Basic Pattern Matching with match

A match compares a value against multiple patterns:

fn main() {
    let number = 3;

    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        _ => println!("Something else"), // _ is a "catch-all" pattern
    }
}


Rust checks patterns top to bottom.

_ means "anything else" (like a default case).

2. Matching Multiple Values

You can match several values with | (OR pattern):

fn main() {
    let number = 2;

    match number {
        1 | 2 => println!("One or Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
}

3. Matching Ranges

You can match ranges of numbers or characters with ..=:

fn main() {
    let number = 7;

    match number {
        1..=5 => println!("Between 1 and 5"),
        6..=10 => println!("Between 6 and 10"),
        _ => println!("Greater than 10"),
    }
}


Works with characters too:

fn main() {
    let letter = 'b';

    match letter {
        'a'..='z' => println!("Lowercase letter"),
        'A'..='Z' => println!("Uppercase letter"),
        _ => println!("Not a letter"),
    }
}

4. Matching and Binding with @

You can capture (bind) a matched value using @:

fn main() {
    let age = 25;

    match age {
        n @ 13..=19 => println!("Teenager, age = {}", n),
        n @ 20..=30 => println!("Young adult, age = {}", n),
        _ => println!("Other age"),
    }
}


Here, n stores the matched value.

5. Matching on Enums

Rust enums are commonly matched using match:

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn main() {
    let shape = Shape::Circle(5.0);

    match shape {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => println!("Rectangle {} x {}", w, h),
    }
}

6. Matching with Structs and Destructuring

You can destructure structs inside match:

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 10, y: 20 };

    match p {
        Point { x, y: 0 } => println!("On the x-axis at {}", x),
        Point { x: 0, y } => println!("On the y-axis at {}", y),
        Point { x, y } => println!("Point at ({}, {})", x, y),
    }
}

7. Matching with if conditions (Guards)

You can add conditions to patterns using if (called guards):

fn main() {
    let number = 4;

    match number {
        n if n % 2 == 0 => println!("Even"),
        _ => println!("Odd"),
    }
}

8. if let for Simpler Matching

When you only care about one pattern, use if let:

fn main() {
    let some_value = Some(42);

    if let Some(x) = some_value {
        println!("Value is {}", x);
    } else {
        println!("No value");
    }
}


Equivalent to:

match some_value {
    Some(x) => println!("Value is {}", x),
    None => println!("No value"),
}

9. while let Loops

Pattern matching can be used in loops:

fn main() {
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("Popped {}", top);
    }
}

10. Matching in Function Parameters

You can match directly in function arguments:

fn print_point(Point { x, y }: Point) {
    println!("x = {}, y = {}", x, y);
}

fn main() {
    let p = Point { x: 5, y: 10 };
    print_point(p);
}

Easy Analogy for Pattern Matching

Think of pattern matching like filters for a package:

You get a box (the value).

You check different shapes of boxes (patterns).

When one matches, you run the matching code.





fn main() {
    let number = 2;

    match number {
        1 | 2 => println! ("one or two"),
        3 => println!("print the number is three" )
    }
}
 // | refter to the or thing either 1 or royjrt 2

fn main() {
    let day = 'A'; // Example: day is the character 'A'

    match day {
        'A'..='K' => println!("The letter is between A and K"),
        'K'..='Z' => println!("The letter is between K and Z"),
        _ => println!("The letter does not exist"),
    }
}



3. Matching Ranges

fn main({
    let number = 7

    match number (
        1..=5 => println!("the number is between one and 5"),
        5..=10 => println!("the number is between 5 and 10"),
        _ => println!("the number is greater than 10"),
    }
}