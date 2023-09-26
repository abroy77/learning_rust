fn main() {
    println!("Hello, world!");

    // let's look at scope
    {
        // s not available here
        let s = 5; // s available here
        println!("{s}");
    } // scope of s is over. s does not exist
      // after the curly brace. Rust calls drop and frees up heap memory if was dynamic var

    // string type is on heap
    // allocated at runtime
    //dynamic and can grow / shrink

    let mut s: String = String::from("hello");

    s.push_str(", world!"); // append
    println!("{}", s); // This will print `hello, world!`

    // fixed size datatypes like arrays, string literals, ints, floats, chars
    // are stored in the stack because their required memory is known at compile time
    // With dynamic datatypes like String and Vectors, the data is stored in the heap
    // until the scope of the initialisation is exited

    // differences in stack and heap copying
    let x = 6;
    let y = x;
    // here the value 5 is copied to y and the stack has 2 independent memory slots for each.
    // changing one does not change the other
    println!("x= {x}, y= {y}");

    let s1 = String::from("hello");
    let _s2 = s1;
    // is passed by ref. now both point to same location on heap

    // to prevent double freeing up of memory (s1 and s2) when we leave scope
    // immediately when s2 is initialised, s1 is made invalid.
    // next line won't work
    // println!("{}", s1)

    // SO WE CALL THAT ^^^^ a MOVE. instead of a copy. cuz instead of
    // copying s1 to s2 and keeping both
    // Rust moves s1 to s2. s1 no longer exists

    // you must deep copy to make 2 separate copies on the heap that
    // are independent. although this is expensive

    // clone can deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // no diff between deep and shallow copy on stack. since working
    // on stack is cheap. no move either. it just copies deep

    // passing to FUNCTIONS!!

    // stack vars are copied
    // heap vars are moved

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    println!("{x}"); // still works

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    println!("{}", s1);

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // here s2 is invalid. s3 works though
    println!("main function s3 {}", s3);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
    // this bit was clunky because we pass s1, it gets burnt
    // then we need to return it in our calculate_length function.

    // tedious.

    // next lesson transferring ownership, and references
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

// example of returning multiple values in a tuple.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
