fn main() {
    let mut s = String::new();
    s = "initial contents".to_string();
    println!("s = {}", s);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s = {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // takes slice so does not take ownership
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {}", s);
    // Macro takes slices not ownwership
}
