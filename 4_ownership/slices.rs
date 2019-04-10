// Version 1:
// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s); // word will get the value 5
//     println!("{}", word);
//     s.clear();
//
//     // word still has the value 5 here, but there's no more string that
//     // we could meaningfully use the value 5 with. word is now totally invalid!
// }
//
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     // Creates a tuple from the string (array position, reference to value)
//     // Returns the array position of the first space
//     for(i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     // If the string of characters has no spaces, return length of string.
//     s.len()
// }

// Version 2:
fn main() {
    let my_string = String::from("hello word");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word (&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax
    let word = first_word(my_string_literal);

    println!("The first word is: {}", word);
}

// Returns a slice to the first first instead of a length
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Slices from start to the current position
            return &s[0..i];
        }
    }

    // Returns slice of entire string
    &s[..]
}
