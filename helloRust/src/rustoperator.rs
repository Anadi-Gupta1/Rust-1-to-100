Rust Operators
Operators
Operators are used to perform operations on values and variables.

Rust supports many common operators, like:

Arithmetic Operators
Assignment Operators
Comparison Operators
Logical Operators
Arithmetic Operators
Arithmetic operators are used to do basic math:

Operator	Name	Example	Result
+	Addition	5 + 3	8
-	Subtraction	5 - 3	2
*	Multiplication	5 * 3	15
/	Division	10 / 2	5
%	Remainder (modulus)	10 % 3	1
Example
fn main() {
  let add = 5 + 3;
  let sub = 10 - 4;
  let mul = 6 * 2;
  let div = 12 / 3;
  let rem = 10 % 3;

  println!("Add: {}", add);
  println!("Sub: {}", sub);
  println!("Mul: {}", mul);
  println!("Div: {}", div);
  println!("Rem: {}", rem);
}
Assignment Operators
Assignment operators are used to assign and update values:

Operator	Example	Same As
=	x = 5	Assign 5 to x
+=	x += 3	x = x + 3
-=	x -= 2	x = x - 2
*=	x *= 4	x = x * 4
/=	x /= 2	x = x / 2
%=	x %= 2	x = x % 2
Example
fn main() {
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
}
Comparison Operators
Comparison operators compare values and return true or false:

Operator	Meaning	Example
==	Equal to	5 == 5 is true
!=	Not equal to	5 != 3 is true
>	Greater than	7 > 3 is true
<	Less than	2 < 5 is true
>=	Greater than or equal to	5 >= 5 is true
<=	Less than or equal to	3 <= 4 is true
Example
fn main() {
  let a = 5;
  let b = 10;

  println!("5 == 10: {}", a == b);
  println!("5 != 10: {}", a != b);
  println!("5 < 10: {}", a < b);
  println!("5 >= 10: {}", a >= b);
}
Logical Operators
Logical operators are used to work with boolean values:

Operator	Name	Description
&&	AND	true if both values are true
||	OR	true if at least one is true
!	NOT	inverts the boolean value
Example
fn main() {
  let logged_in = true;
  let is_admin = false;

  println!("Is regular user: {}", logged_in && !is_admin);
  println!("Has any access: {}", logged_in || is_admin);
  println!("Not logged in: {}", !logged_in);
}