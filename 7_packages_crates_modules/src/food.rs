mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // Fields are private by default, accessing the private id value will error
    // println!("The ID is {}", v.id);

    // Enum variants are public by default
    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}
