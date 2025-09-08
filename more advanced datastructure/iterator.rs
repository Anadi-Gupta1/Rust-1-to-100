The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself.

In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up. For example, the code in Listing 13-10 creates an iterator over the items in the vector v1 by calling the iter method defined on Vec<T>. This code by itself doesn’t do anything useful.

let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();



apn vector and HasMap ko iterator mai save kar skte hai

iterating using loops

fn main(){
    let v1 = vec![1,2,3];
    for val in v1.iter(){
        println!("{}", val);
    }
}
fn main(){
    let v1 = vec![1,2,3];
    for nums in v1 {
        println!("{}", nums);
    }
}

iterating after creatinging iterator variable
fn main(){
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }
}
v1_iter is just borrowing the value of v1 so v1 is still valid here

iterator using next method
fn main(){
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    println!("{:?}", v1_iter.next());