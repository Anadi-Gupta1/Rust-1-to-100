/*
Rust Operators
Operators are used to perform operations on values and variables.

Rust supports many common operators, like:

Arithmetic Operators
Assignment Operators  
Comparison Operators
Logical Operators
*/

fn main() {
    println!("=== ARITHMETIC OPERATORS ===");
    
    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: 5 + 3 = {}", add);
    println!("Sub: 10 - 4 = {}", sub);
    println!("Mul: 6 * 2 = {}", mul);
    println!("Div: 12 / 3 = {}", div);
    println!("Rem: 10 % 3 = {}", rem);

    println!("\n=== ASSIGNMENT OPERATORS ===");
    
    let mut x = 10;
    println!("Start: {}", x);

    x += 5;
    println!("After += 5: {}", x);

    x -= 2;
    println!("After -= 2: {}", x);

    x *= 2;
    println!("After *= 2: {}", x);

    x /= 3;
    println!("After /= 3: {}", x);

    x %= 4;
    println!("After %= 4: {}", x);

    println!("\n=== COMPARISON OPERATORS ===");
    
    let a = 5;
    let b = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 > 10: {}", a > b);
    println!("5 >= 10: {}", a >= b);
    println!("5 <= 10: {}", a <= b);

    println!("\n=== LOGICAL OPERATORS ===");
    
    let logged_in = true;
    let is_admin = false;

    println!("Is regular user (logged in AND not admin): {}", logged_in && !is_admin);
    println!("Has any access (logged in OR admin): {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);
}