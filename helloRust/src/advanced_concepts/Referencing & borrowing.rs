Conditions for borrowing and referencing

```
owner => rihana

owner => rihana
borrower1
borrower2
borrower3

owner => rihana
borrower1 => hanky panky
```


Jargon #3 - Borrowing and references
Rihana upgrades
Rihana now says I’d like to be borrowed from time to time. I will still have a single owner, but I can still be borrowed by other variables temporarily. What rules do you think should apply to her?
She can be borrowed by multiple people that she’s friends with but does no hanky panky
If she does want to do hanky panky, she can only have 1 borrower that she does it with. She cant simultaneously be with other borrowers (even with no hanky panky)
 
notion image
References
References mean giving the address of a string rather than the ownership of the string over to a function
For example
fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;

    println!("{}", s2);
    println!("{}", s1);    // This is valid, The first pointer wasn't invalidated
}

notion image
Borrowing
You can transferring ownership of variables to fns. By passing a reference to the string to the function take_ownership, the ownership of the string remains with the original variable, in the mainfunction. This allows you to use my_stringagain after the function call.
fn main() {
    let my_string = String::from("Hello, Rust!");
    takes_ownership(&my_string);  // Pass a reference to my_string
    println!("{}", my_string);    // This is valid because ownership was not transferred
}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string);  // some_string is borrowed and not moved
}

Mutable references
What if you want a function to update the value of a variable?

fn main() {
    let mut s1 = String::from("Hello");
    update_word(&mut s1);
    println!("{}", s1);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}

Try having more than one mutable reference at the same time - 
fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    update_word(&mut s1);
    println!("{}", s1);
    println!("{}", s2);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}

Rules of borrowing
There can me many immutable references at the same time

fn main() {
    let  s1 = String::from("Hello");
    let s2 = &s1;
    let s3 = &s1;
    
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
}
// No errors

There can be only one mutable reference  at a time
fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    let s3 = update_word(&mut s1);
    
    println!("{}", s1);
    println!("{}", s2);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}
// Error

If there is a mutable reference , you can’t have another immutable reference either.
fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    let s3 = &s1;
    
    println!("{}", s1);
    println!("{}", s2);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}

 
This to avoid any data races/inconsistent behaviour
If someone makes an immutable reference , they don’t expect the value to change suddenly
If more than one mutable references happen, there is a possibility of a data race and synchronization issues
 
💡
Two good things to discuss at this point should be but we’re going to ignore it for now
1. Lifetimes
2. String slices (&str)
Ref - https://doc.rust-lang.org/book/ch04-03-slices.html
 