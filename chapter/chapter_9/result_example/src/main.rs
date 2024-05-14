use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // 9-1
    // let f = File::open("hello.txt");
    // let file = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     }
    // };
    // 9-2
    // let file = File::open("hello.txt").unwrap();
    // 9-3
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // 9-4
    // let f = File::open("hello.txt");
    // let file = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Failed to create hello.txt:{:?}", e),
    //         },
    //         other_error => panic!("Failed to open hello.txt:{:?}", other_error),
    //     },
    // };
    // 9-5
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Failed to create hello.txt: {:?}", error);
    //         })
    //     } else {
    //         panic!("Failed to open hello.txt: {:?}", error);
    //     }
    // });
    // 9-6
    println!("content:{}", read_from_file().unwrap());
}
// fn read_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut file = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut s = String::new();
//     match file.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }
// fn read_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt")?;
//     let mut s = String::new();
//     match file.read_to_string(&mut s)?;
//     Ok(s)
// }
fn read_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
