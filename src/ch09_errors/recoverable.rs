use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>>{
    // let f = File::open("hello.txt");
    //
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    //         },
    //         other_error => panic!("There was a problem opening the file: {:?}", other_error),
    //     },
    // };
    //
    // // Alternative form of the above
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Tried to create file but there was a problem: {:?}", error);
    //         })
    //     } else {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     }
    // });

    // 2. Shortcuts for Panic on Error

    // Unwrap returns the Ok variant by default. If Err it will panic!
    // let f = File::open("hello.txt").unwrap();

    // .expect(...) allows you to specify the error message
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // 4. allowing for the use of ? in main
    let f = File::open("hello.txt")?;

    Ok(());
}

// 3. Propogating errors - long way
fn read_username_from_file_long_way() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 3. Propogating errors - short way using ?
fn read_username_from_file_short_way() -> Result<String, io::Error> {
    // If results is an Err, function returns Err
    // Otherwise returns Ok(Result)
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorter_way() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_shortest_way() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
