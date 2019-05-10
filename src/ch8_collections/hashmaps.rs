use std::collections::HashMap;

fn main() {
    // 1. Basic
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 2. From existing tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let _scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 3. Ownership
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are now invalid because the String value is moved.
    // println!("My favorite color is {}.", field_value);

    // Error is:
    // = note: move occurs because `field_value` has type `std::string::String`, which does not implement the `Copy` trait

    // Types that implement Copy won't have this error, such as i32
    let field_name2 = String::from("Favorite Number");
    let field_value2 = 27;

    let mut map2 = HashMap::new();
    map2.insert(field_name2, field_value2);

    println!("3. My favorite number is {}.\n", field_value2);

    // no errors!

    // 4. Accessing values
    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);

    // iterate over the map
    for (key, value) in &scores {
        println!("4. {}: {}", key, value);
    }
    println!("");

    // 5. Updating a HashMap
    let mut scores3 = HashMap::new();

    // Default behavior of insert is to overwrite
    scores3.insert(String::from("Blue"), 10);
    scores3.insert(String::from("Blue"), 25); // 10 has been overwritten

    println!("5.a. {:?}\n", scores3);

    // Only insert if key has no value
    scores3.entry(String::from("Yellow")).or_insert(50);
    scores3.entry(String::from("Blue")).or_insert(75); // is not inserted

    // scores3.entry(...) returns an Entry to the map, which is a single key-value relationship

    println!("5.b. {:?}\n", scores3);

    // 6. Updating a value based on the old value
    let text = "world oh world oh beautiful world";
    let mut text_map = HashMap::new();

    for word in text.split_whitespace() {
        // Inserts zero if word is not in map.
        // Always returns mutable entry to value in the map.
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("6. {:?}\n", text_map);
}
