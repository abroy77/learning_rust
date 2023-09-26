fn main() {
    // let's learn to pass references woo
    //ref like pointer
    // unlike a pointer! "reference is guaranteed
    // to point to a valid value of a particular type
    //  for the life of that reference."

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    // The &s1 syntax lets us create a reference that
    // refers to the value of s1 but does not own it. Because
    // it does not own it, the value it points to will not
    // be dropped when the reference stops being used.

    // call function to change mutable ref
    let mut s = String::from("hello");

    change(&mut s);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
  // passing reference is called borrowing
  //  no full ownership transfer

// references are immutable by default.
//  can't change the borrowed value

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
//  Mutable references have one big
// restriction: if you have a mutable reference to a
// value, you can have no other references to that value.
// This code that attempts to create two mutable references
// to s will fail:

// references cease to exist after their last use

//Let’s recap what we’ve discussed about references:

// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.
