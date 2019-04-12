// Defines a module named 'sound'
mod sound {
    pub mod instrument {

        pub mod woodwind {
            fn breath_in() {
                println!("** inhales **");
            }

            pub fn clarinet() {
                breath_in();

                println!("Toot toot toot!");
            }
        }

        fn guitar() {
            // Function body goes here
        }
    }

    mod voice {

    }
}

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::woodwind::clarinet();
        instrument::woodwind::clarinet();
        instrument::woodwind::clarinet();
    }
}

// Absolute path
// use crate::sound::instrument::woodwind;

// Relative path
// use self::sound::instrument::woodwind;

fn main() {
    println!("Let's begin.");

    // Absolute path
    // crate::sound::instrument::woodwind::clarinet();

    // Relative path
    // sound::instrument::woodwind::clarinet();

    performance_group::clarinet_trio();
    performance_group::instrument::woodwind::clarinet();
}
