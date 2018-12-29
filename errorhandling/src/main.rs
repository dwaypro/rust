use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::fs;

fn main() {
    println!("Hello, world!");
    




    let f = File::open("hello.txt").expect("failed to open hello.txt");

    // let f = match f{
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt"){
    //             Ok(fc) =>fc,
    //             Err(e) => panic!("Tried to create file but there was a problem: {:?}", e)
    //         },
    //         other_error => panic!("There was a problem opening the file :{:?}", other_error)
    //     },
    // };

    read_username_from_file();



}

//v1
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

//v2
// fn read_username_from_file() -> Result<String, io::Error>{
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

//v3
// fn read_username_from_file() -> Result<String, io::Error>{
//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

//v4
fn read_username_from_file() -> Result<String, io::Error>{
     fs::read_to_string("hello.txt")
}