fn main() {
    println!("Variables tutorial");
    // variables ans mutability
    /*
     * By default variables are immutable
     * `mut` keyword qualifier is used to add mutable quality to a variable
     */

    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The valye of x is: {x}");

    // Constants
    // mut can not be used with a constant. constant are always immutable

    // const FIFTEEN_HOURS_IN_SECONDS: u32 = 15 * 60 * 60;

    /*
     * Shadowing
     *
     * declaring a new variable with an existing variable name
     * we shadow a variable by using the same variable name and repeating the use of the let
     * keyword
     */
    let v = 0;

    let v = v + 1;

    {
        let v = v * 2;
        println!("The value of v in the inner scope is {v}");
    }
    println!("The value of v in the outer scope is {v}");

    // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    // with shadowing we can also change the type of the variable when it is shadowed
    //

    /*
     * Data types
     * Subsets:
     *      Scalar: integers, floating-point numbers, Booleans, and characters.
     *      Compound:
     *
     * Integers:
     *  signed: i8, i16, i32, i64, i128, isize
     *  unsigned: u8, u18, u32, u64, u128, usize (size of the OS arch)
     *
     *  Numerical literal:
     *      Decimal: 98_222
     *      Hex: 0xff
     *      Octal: 0o77
     *      Binary: 0b1011101
     *      Byte: b'A'
     *
     * Floatin-point:
     *  f32 & f64
     *      default type for floating numbers are f64
     *
     *
     * Boolean type:
     *      true , false
     *
     * Character Type
     *   `char`
     *   char literals are represented with single quotes as oppose to string literals
     *   let c = 'A';
     *   char type are 4 byte. they represent a unicode scalar value which means they can represent a lot more than ASCII
     *
     * Compound types:
     *  Tuple:
     *      A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
     *      Tuples have a fixed length: once declared, they cannot grow or shrink in size.
     *      We create a tuple by writing a comma-separated list of values inside parentheses.
     *      Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
     *      let tup: (i32, f64, u8) = (500, 6.4, 1);
     *      let tup = (500, 6.4, 1);
     *      let (x, y, z) = tup;
     *      let five_hundred = tup.0;
     *      let six_point_four = tup.1;
     *      let one = tup.2;
     *
     *  Array:
     *      Unlike a tuple, every element of an array must have the same type.
     *      Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.
     *      let a = [1, 2, 3, 4, 5];
     *      let a: [i32; 5] = [1, 2, 3, 4, 5];
     *      let a = [3; 5]; // [3, 3, 3, 3, 3]
     *      let first = a[0];
     *      let second = a[1];
     *      let index = 10;
     *      let element = a[index]; // this will cause a panic
     *
     *      let a = [1, 2, 3, 4, 5];
     *      let index = 10;
     *      let element = a.get(index); // this will return None
     *
     *      let a = [1, 2, 3, 4, 5];
     *      let index = 10;
     *      let element = a.get(index).expect("Index out of bounds"); // this will return None
     *
     *
     */

    /*
     * Functions:
     *  the fn keyword allows you to declare new functions.
     *
     *  Parameters
     *      We can define functions to have parameters, which are special variables that are part of a function’s signature.
     *
     *  // Example
     *
     *  fn print_labeled_measurement(value: i32, unit_label: char) {
     *      println!("The measurement is: {value}{unit_label}");
     *  }
     *
     *  return value definition
     *
     *  fn five() -> i32 {
     *      5
     *  }
     */

    /*
     * Statements and Expressions
     *  Function bodies are made up of a series of statements optionally ending in an expression
     *
     *  Statements are instructions that perform some action and do not return a value.
     *  Expressions evaluate to a resulting value.
     *
     *  let y = 5; // This is a statement
     *
     *  x + 1 // this is an expression that returns the result of the addition
     *  // expressions do not include ending semicolon
     *
     *  { let x = 3; x + 1 } // This is also an expression, because of the expression ending the
     *  block scope
     *
     *  expression at the end of a function is taking as the return value
     *
     */

    /*
     * Ownership
     * Set of rules that govern how rust manages memory
     *
     *  Ownership rules
     *  ***************
     *  - Each value in Rust has on owner.
     *  - There can only be one owner at a time
     *  - When the owner goes out of scope, the value is dropped
     *
     * Variable scope
     * **************
     * The scope is the range within a program for which an item is valid.
     *  A variable is valid from the point at which it is declared until the end of the block in which it is declared.
     *  The block is a collection of statements within curly braces.
     *
     *  let x = 5;
     *  {
     *      let y = 6;
     *      println!("The value of y is: {y}");
     *  }
     *  println!("The value of x is: {x}");
     *
     *  The value of y is only valid within the block scope
     *
     *  Ownership and functions
     *  ***********************
     *  Passing a variable to a function will transfer the ownership of the variable to the function
     *  The variable will be dropped when the function goes out of scope
     *
     *  fn main() {
     *      let s = String::from("hello");
     *      takes_ownership(s);
     *      println!("The value of s is: {s}");
     *  }
     *
     *  fn takes_ownership(some_string: String) {
     *      println!("The value of some_string is: {some_string}");
     *  }
     *
     *  The value of s is only valid within the main function
     *
     *  Memory and Allocation
     * ***********************
     * When a variable goes out of scope, Rust calls a special function for us.
     * This function is called drop, and it’s where the author of String can put the code to return the memory.
     * Rust calls drop automatically at the closing curly bracket
     *
     *  Returning values from functions
     *  *******************************
     *  Returning values from a function will transfer the ownership of the variable to the calling function
     *
     *  fn main() {
     *      let s1 = gives_ownership();
     *      let s2 = String::from("hello");
     *      let s3 = takes_and_gives_back(s2);
     *  }
     *
     *  fn gives_ownership() -> String {
     *      let some_string = String::from("hello");
     *      some_string
     *  }
     *
     *  fn takes_and_gives_back(a_string: String) -> String {
     *      a_string
     *  }
     *
     *  The value of s2 is no longer valid after the call to takes_and_gives_back
     *
     *
     */

    // double free error // move error
    {
        let s1 = String::from("hello");
        let _s2 = s1; // move is performed to prevent double free error
                      /*
                       * s1 as been moved to _s2 so s1 is no longer  valid
                       */
        // println!("{}, world!", s1); // error
        println!("{}, world!", _s2); // valid
    }

    {
        //clone - deep copy
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    /*
     * Stack-Only Data: Copy
     * *********************
     * types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out
     * Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack
     */

    /*
     *  Ownership and functions
     *  ***********************
     *  The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy
     */

    /*
     *  References and Borrowing
     *  ************************
     *  We can pass a reference to a function instead of passing the ownership of the variable
     *  We call the action of creating a reference borrowing
     *
     *  passing value by reference syntax
     *  let str = String::from("hello");
     *  let len = calculate_length(&str);
     *
     *  Mutable References - &mut
     *  *****************
     *  We can also create mutable references to a variable
     *  let mut s = String::from("hello");
     *  change(&mut s);
     *
     *  We can only have one mutable reference to a variable in a scope
     *
     *  Dangling References
     *  *******************
     *  A dangling reference is a pointer that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory.
     *  Rust ensures that references will never be dangling references
     *
     *  let reference_to_nothing = dangle();
     *
     *  fn dangle() -> &String {
     *      let s = String::from("hello");
     *      &s
     *  }
     *
     *  The value of s is dropped when the function goes out of scope
     *
     */

    /*
     *  Slices
     *  ******
     *  Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection
     *  Slice is a kind of reference so it has no ownership of the variable
     *
     *  let s = String::from("hello world");
     *  let hello = &s[0..5];
     *  let world = &s[6..11];
     *
     *  let s = String::from("hello");
     *  let slice = &s[0..2];
     *  let slice = &s[..2];
     *  let slice = &s[3..s.len()];
     *  let slice = &s[3..];
     *
     * String Litrerals as slice
     * let s = "Hello world"
     * type of is here is &str, it is a slice pointing to the specific point of the binary. it is
     * an immutable refrence
     *
     */

    /*
     *  Structs
     *  *******
     *  Structs are used to create custom data types
     *
     *  struct User {
     *      username: String,
     *      email: String,
     *      sign_in_count: u64,
     *      active: bool,
     *  }
     *
     *  let user1 = User {
     *      username: String::from("someusername"),
     *      email: String::from("someemail"),
     *      sign_in_count: 1,
     *      active: true,
     *  };
     *
     *  let user2 = User {
     *      username: String::from("anotherusername"),
     *      email: String::from("anotheremail"),
     *      ..user1
     *  }
     *
     */
    /*
     *  Tuple structs without named fields to create different Types
     *  struct Color(i32, i32, i32);
     *  struct Point(i32, i32, i32);
     *
     *  let black = Color(0, 0, 0);
     *  let origin = Point(0, 0, 0);
     *
     *  Unit-Like structs -> this are structs without any fields
     *  Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter
     */

    /*
     *  Methods
     *  *******
     *  Methods are similar to functions but they are defined within the context of a struct
     *
     *  struct Rectangle {
     *      width: u32,
     *      height: u32,
     *  }
     *
     *  impl Rectangle {
     *      fn area(&self) -> u32 {
     *          self.width * self.height
     *      }
     *  }
     *
     *  let rect1 = Rectangle { width: 30, height: 50 };
     *  println!("The area of the rectangle is {} square pixels.", rect1.area());
     *
     *
     */

    /*
     *  Associated Functions
     *  ********************
     *  Associated functions are functions that are associated with the struct
     *  They are called using the :: syntax
     *
     *  impl Rectangle {
     *      fn square(size: u32) -> Rectangle {
     *          Rectangle { width: size, height: size }
     *      }
     *  }
     *
     *  let sq = Rectangle::square(3);
     *
     */

    /*
     *  Enums
     *  *****
     *  Enums allow you to define a type by enumerating its possible variants
     *
     *  enum IpAddrKind {
     *      V4,
     *      V6,
     *  }
     *
     *  let four = IpAddrKind::V4;
     *  let six = IpAddrKind::V6;
     *
     *  You can also attach data to the variants
     *
     *  enum IpAddr {
     *      V4(String),
     *      V6(String),
     *  }
     *
     */
}
