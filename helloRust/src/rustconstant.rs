Rust Constants
Constants
Constant variables are used to store values that never change.

Unlike regular variables, constants must be defined with a type (e.g. i32 or char).

Creating a Constant
To create a constant, use the const keyword, followed by the name, type, and value:

Example
const BIRTHYEAR: i32 = 1980;
const MINUTES_PER_HOUR: i32 = 60;
Constants Must Have a Type
You must write the type when creating a constant. You cannot let Rust guess the type like you can with regular variables:

Example
const BIRTHYEAR: i32 = 1980; // Ok
const BIRTHYEAR = 1980; // Error: missing type
Naming Rules
Another thing about constants, is that it is considered good practice to declare them with uppercase.

It is not required, but useful for code readability and common for Rust programmers:

Examples:

MAX_SPEED
PI
MINUTES_PER_HOUR
Constants vs Variables
Here's a quick comparison:

Feature	Constant (const)	Variable (let)
Can change?	No	Yes, if mut is used
Type required?	Yes	No (optional)