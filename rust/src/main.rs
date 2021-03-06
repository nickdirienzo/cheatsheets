// Can build and run with:
// 1. cargo build
// 2. cargo run

fn main() {
    let x = 7; // Rust has type inference
    // x = 10; // And vars are immutable by default

    let (y, z) = (1, 2); // And has pattern matching. Tuple deconstructing.

    let mut a = 1; // This is mutable
    a = 2;

    let mut b: i32; // This is mutable and has a type of "signed 32-bit int"
                    // It's also not assigned, so this will warn at compile-time.
    // println!("{}", b); // If we try to use b without assignment, we error on compile.
    
    if x == 5 {
        println!("x is 5!");
    } else if x == 6 {
        println!("x is 6!");
    } else {
        println!("x is not 5 or 6");
    }

    // We can also do this. Which is super awesome.
    // if is an expression.
    let c = if x == 5 { 10 } else if x == 6 { 15 } else { 20 }; // c: i32 (also, note the semicolon).
    println!("{}", c);

   // let is a statement 
   let d = x; // d is of type (), the unit type
   // ; turn expressions into statements by throwing away the value and returing ()

   print_num(a); // functions
   print_sum(a, x);
   print_num(magic(a));
   print_num(prod(x));

   // a = die(); // diverging functions (return-type !) can be used as any type
   let t: (i32, &str) = (1, "Hello, world."); // Tuples. They don't have to be typed.
   // We can assign same-size, same-type tuples into each other, if mut.
   let mut u = t;
   if t == u {
        println!("match");
   } else {
        println!("does not match.");
   }

   let origin: Point = Point{ x: 0, y: 0 }; // immutable
   println!("Origin: ({}, {})", origin.x, origin.y);

   let mut origin2: Point = Point{ x: 0, y: 0 }; // mutable. Individual fields cannot be mutable?
   origin2.x = 1;
   println!("Origin: ({}, {})", origin2.x, origin2.y);

   struct Inches(i32); // newtype (size-1 tuple-struct)
   let length = Inches(10);
   let Inches(in_len) = length; // Unpacks the tuple into in_len
}

// void function. takes in a param of type i32 named x.
fn print_num(x: i32) {
    println!("Num is {}", x);
}

fn print_sum(x: i32, y: i32) { // Must always declare param types
    println!("Num is {}", sum(x, y));
}

fn sum(x: i32, y: i32) -> i32 { // returns type i32. I really like this syntax.
    x + y
}

fn magic(x: i32) -> i32 {
    if x < 5 { return x; } // early returns are allowed
    x + 1 // although it is the norm to leave off return of what you are returning generally
}

// The more Rust-ful way of writing magic
fn prod(x: i32) -> i32 {
    if x < 5 {
        x
    } else {
        x + 1
    }
}

/// `die` is a diverging function. We know this from !.
/// 
/// # Markdown
/// 
/// Rust doc comments support it.
fn die() -> ! { // ! denotes does not return
    panic!("Whelp, we're dead.");
}

// We can return multiple values!
fn two(x: i32) -> (i32, i32) {
    (x, x + 1)
}

struct Point {
    x: i32,
    y: i32,
}

struct Point2 {
    //mut x: i32, // Individual fields cannot be mutable :(
    y: i32,
}
