fn main() {
    // Create string using 'new' 
    let mut s = String::new();

    // Create string from existing contents
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s2 = "initial contents".to_string();

    // Alternatively:
    let s3 = String::from("initial contents");

    // Strings are UTF-8 encoded, so we can use special characters:
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Modifying strings
    let mut s4 = String::from("foo");
    s4.push_str("bar"); // takes a slice so as to not take ownership of s2
    printf("{}", s);

    s4.push("!");

    // Using the + operator
    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6; // note s1 has been moved here and can no longer be used  
    // Signature of + operator is: fn add(self, s: &str) -> String {
    // so s5 is no longer valid after the appending

    let st1 = String::from("tic");
    let st2 = String::from("tac");
    let st3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // returns "tic-tac-toe"
    // returns a String without taking ownership of any parameters
    
    // String slices
    // * Can't index directly due to language funkiness
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // returns Зд

    // Accessing first byte would panic at runtime
    // let s2 = &hello[0..1];

    // Iterating over strings: 
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
