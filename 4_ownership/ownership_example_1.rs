fn main() {
    // gives_ownership moves its return value into s1
    let s1 = gives_ownership();

    // s2 comes into scope
    let s2 = String::from("hello");

    // s3 is move into takes_and_gives_back, which also moves its return value
    // into s3
    let s3 = takes_and_gives_back(s2);

    println!("s3: {}", s3);
    println!("s1: {}", s1);
} // s3 goes out of scope and is dropped.
// s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped


fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
