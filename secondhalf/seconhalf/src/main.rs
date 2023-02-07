use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // reading a file with rust
    let mut read_file = File::open("read.txt").expect("cant open file");
    let mut string_data = String::new();
    read_file
        .read_to_string(&mut string_data)
        .expect("Opps cannot read file");

    let args: Vec<String> = env::args().collect();

    for arguments in args.iter() {
        println!("{}", arguments)
    }

    // writing to file in rust
    let mut create_file = File::create("createdFile.txt").expect("could not creacte the file");

    create_file
        .write_all(b"welcome home mate")
        .expect("sorry mate somthing went wrong");

    // println!("{}", string_data);

    // rust switch concept
    let val = 12;

    match val {
        1 => println!("hello world 1"),
        12 => println!("hello world 12"),
        _ => println!("hello world"),
    }

    // 
    
}
