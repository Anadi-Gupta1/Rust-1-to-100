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
