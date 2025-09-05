what is Strings
sting is a data which consist of bunch of byte
for exammple anadi so a n a d i 

The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust, they might be referring to either the String or the string slice &str types, not just one of those types. Although this section is largely about String, both types are used heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

String slices let you reference a contiguous sequence of elements in a string


CREATING A String

fn main(){
    let name = String::from("anadi");
    println!("{}",name)
}

MUTATING A String
fn main(){
    let mut name = String::from("M'21");
    name.push_str("anadi");
    println!("{}",name)
}
output M'21anadi


DELETE A String
fn main(){
    let mut name = String::from("M'21");
    name.push_str("anadi");
    println!("{}",name);
    name.replace_range(0..3,"2021");
    println!("{}",name)
}


if you want sort of view to see in the string which is know as string slice
if you want to see the first 3 character of the string then it can be done by slice only particular part of the string

write a function that takes a string and return the first word of the string



now here commes my the main part what we are doing right now is 
we are creating a original string and then we are creating a slice of that string
and using that another variable to store the slice of the original string
now the problem statement got changed now what i want is



i want that i have to use slice string only such that
i use the original string and then i have to return the slice of that string
i dont want to create another variable to store the slice of the original string
this is the reason why we are using slice here
now i want that user input the string and then i have to return the first word of that string by string slicing

fn main(){
    let name = String::from("M'21 anadi");
    let name2 = &name[0..5]; // here i am taking reference of the original string
    println!("the first word is {}",name2) 

    word.clear
}

Output the first word is M'21
