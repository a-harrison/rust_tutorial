// Version 1:

// fn main() {
//     let s1 = String::from("hello");
//     let (s2, len) = calculate_length(s1);
//
//     println!("The length of '{}' is {}.", s2, len);
// }
//
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len returns the size of a String
//
//     (s, length)
// }

// Version 2:

fn main() {
    let s1 = String::from("somewhere over the rainbow");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
