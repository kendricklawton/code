fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");

    let x = 5;
    let y = x;

    println!("y: {y}");

    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2 {s2}");

    // This is intentional
    let mut s3 = String::from("hello");
    println!("s3 before: {s3}");
    s3 = String::from("ahoy");

    println!("s3 after: {s3}, world!");

    let s4 = String::from("hello");
    // When you see a call to clone,
    // you know the some arbitray code
    // is being executed and that code may by expensive.
    let s5 = s4.clone();

    println!("s4 = {s4}, s5 = {s5}");

    // Stack-Only Data: Copy
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    let s6 = String::from("hello");
    println!("s6: {s6}");

    takes_ownership(s5);

    let x2 = 5;
    makes_copy(x2);

    let s7 = gives_ownership();
    println!("s7: {s7}");

    let s8 = String::from("hello");

    let s9 = takes_and_gives_back(s8);
    println!("s9: {s9}");

    let s10 = String::from("hello");
    let (s11, len) = calculate_length(s10);
    println!("The length of '{s11}' is {len}");

    // Using a function that has a reference to an object
    let s12 = String::from("hello");
    let len2 = calculate_length_two(&s12);
    println!("the length of '{s12}' is {len2}.");
}
// Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_two(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped.
