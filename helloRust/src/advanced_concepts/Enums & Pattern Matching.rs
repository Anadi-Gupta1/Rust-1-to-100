enum Shape {
    Circle(f64),       // radius
    Rectangle(f64, f64), // width, height
    Square(f64)
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Square(s) => s * s,
    }
}

fn main() {
    let c = Shape::Circle(3.0);
    println!("Area: {}", area(c));
}






enum Direction {
    North,
    South,
    East,
    West,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::North => println!("Moving up"),
        Direction::South => println!("Moving down"),
        Direction::East => println!("Moving right"),
        Direction::West => println!("Moving left"),
    }
}

fn main() {
    let dir = Direction::North;
    move_player(dir);
}


enum Direction {
    North,
    south,
    east,
    west
}
fn get_direction name(dir: Direction) -> &"static str"{
    match dir {
        Direction::North => "North",
        Direction::south => "south",
        Direction::east => "east",
        Direction::west => "west",
    }
}



// What is pattern matching
pattern matching is a type of matching in which the structure and content of data are examined to determine its type and extract relevant information. In Rust, pattern matching is commonly used with enums and can be done using the `match` keyword.
Pattern matching allows you to concisely and safely handle different data types and structures, making your code more readable and maintainable.
Pattern matching matches across various variants and calculates the appropriate response based on the specific variant being matched.




enum Shape {
    Circle(f64),       // radius
    Rectangle(f64, f64), // width, height
    Square(f64)
}


}
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14*radius*radius,
        Shape::Rectangle(width, height) => width*height,
        Shape::Square(side_length) => side_length*side_length,
    }
}
fn main() {
    let shape = Shape::Circle(5.0);
    println!("Area: {}", calculate_area(shape));
    let shape = Shape::Square(4.0);
    println!("Area: {}", calculate_area(shape));
    let shape = Shape::Rectangle(3.0, 6.0);
    println!("Area: {}", calculate_area(shape));
}
return 0;
}

    Pattern matching gives you all the values and expect you to give all the values so you must handle every possible variant.
