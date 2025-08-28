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