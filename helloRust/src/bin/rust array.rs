Rust Arrays
Arrays
Arrays are used to store multiple values in a single variable, instead of declaring separate variables for each value.

Create an Array
You can create an array using square brackets [ ], and separate the values with commas.

Note: Make sure all values are of the same data type (integers in the example below):

Example
let numbers = [1, 2, 3, 4, 5];
This creates an array with five integers.

Access Array Elements
To access an array element, refer to its index number.

Array indexes start with 0: [0] is the first element. [1] is the second element, etc.

This statement accesses the value of the first element [0] in numbers:

Example
let numbers = [1, 2, 3, 4, 5];
println!("The first number is: {}", numbers[0]);
Change Array Values
To change the value of a specified element, refer to the index number and assign a new value.

Remember to make the array mutable (using the mut keyword):

Example
let mut numbers = [1, 2, 3, 4, 5];
numbers[0] = 10;
println!("The new first number is: {}", numbers[0]);
Array Length
You can get the number of elements in an array using the .len() method:

Example
let numbers = [1, 2, 3, 4, 5];
println!("This array has {} elements.", numbers.len());
Loop Through an Array
You can loop through the array elements with the for loop.


Example
let fruits = ["apple", "banana", "orange"];
for fruit in fruits {
  println!("I like {}.", fruit);
}
Print the Entire Array
Note: When printing the whole array, you must use {:?} inside println!:

Example
let numbers = [1, 2, 3, 4, 5];
println!("{:?}", numbers);
If you are just printing one element from the array, you can use {}.

Example
let numbers = [1, 2, 3, 4, 5];
println!("{}", numbers[0]);
To Sum Up:

When printing a single element from an array, use {}.

When printing the entire array, use {:?}.

If you are wondering why we didn't use {:?} inside the loop (in the example earlier on this page), it is because in a loop, you are printing each single value from the array. Since each value is a string, {} works. But to print the whole array, you need to use {:?}.

A good rule is to use {} for basic types like strings, numbers, and booleans, and {:?} for data structures like arrays and vectors - when printing the entire structure.

Fixed Size (Arrays) vs. Dynamic Size (Vectors)
You will often hear the terms fixed size and dynamic size when talking about arrays in Rust.

This is because arrays in Rust have a fixed size, meaning you cannot add or remove elements after the array is created:

Example
// An array with 3 elements
let mut cars = ["Volvo", "BMW", "Ford"];

// Trying to add another element (a fourth element) to the cars array will result in an error
cars[3] = "Mazda";   // Error: index out of bounds
Vectors - Dynamic Size Example
For operations that require adding and removing array elements, you can use Vectors, which are resizable arrays.

The size of a vector is dynamic, meaning it can grow and shrink as needed.

You can use the vec! macro to create a vector:

Example
// A vector with 3 elements
let mut cars = vec!["Volvo", "BMW", "Ford"];

// Add another element
cars.push("Mazda");

println!("{:?}", cars); // ["Volvo", "BMW", "Ford", "Mazda"]