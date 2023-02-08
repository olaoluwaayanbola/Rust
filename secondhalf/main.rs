#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, Write};

const MAX: u8 = 200;
// enum types
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    // Arrays in rust
    let arr = [1,2,3,4,5,6,7];
    for i in 1..arr.len() {
        println!("{}-->arrays", i)
    }

    // Using struct
    let mut val = Color {
        red: 56,
        green: 43,
        blue: 90,
    };
    println!("{} {} {}", val.red, val.green, val.blue);
    // Using refrences in rust you can use ampersand to make refrences
    // you can have multiple refrences
    let mut x = 10;
    // let xr = &x; ---> immutable and mutable cant coexist so
    // to avoid this put it in a codeblock

    // to make a refrence mutable
    let xx = &mut x;
    *xx += 1;

    // solving the above issue
    {
        let xr = &x;
        println!("x is {}", xr);
    }
    // code blocks
    // they allow you create blocks of cde whose values cannot be reached outside

    {
        // this value cannot be used out side the code block
        let y = 10;
        let x = 67;
    }

    // Tuples they can store multiple data types unlike vectors
    let tup1 = (20, false, 30, (1, 2, 3, 4), "banji", vec![1, 2]);
    println!("{}", (tup1.3).2);
    // ------>destructuring a tuple
    let tup2 = (1, 2, "banji");
    let (one, two, banji) = tup2;
    println!("{}", one);

    // using Constants  the constants must always be in capital letter
    for d in 1..MAX {
        println!("i am the main character")
    }

    // i32 is the number of bytes in rust that is 32bits the i infront means it is signed
    // i64 means the number can take 64 bits of space
    // you can also make the the number unsigned that is it will only take negative numbers
    // ---> that is it will be u32 instead of i32
    // f32 floating pointNumber

    let mut x = 4005;
    let boy_girl = true;
    println!("The value of x is {}", x);

    if (boy_girl) {
        println!("its a boy!!");
    } else {
        println!("its a girl!!");
    }

    // controlled variable
    let mut n = 1;

    // ----> loops
    // loop{
    //     n += 1;
    //     println!("hello world");
    //     if(n > 10){
    //         return
    //     }
    // }

    // ----> whileloops
    let mut s = 1;
    while s <= 20 {
        println!("whats up");
        s += 1;
    }

    // for loop
    let number = [1..51];

    for val in number {
        println!("number")
    }
    let palyer_direction: Direction = Direction::Up;

    match palyer_direction {
        Direction::Up => println!("players is going up"),
        Direction::Down => println!("players is going Down"),
        Direction::Left => println!("players is going Left"),
        Direction::Right => println!("players is going Right"),
    }

    // vectors arrays for rust
    // to add use push
    // to remove use remove
    
    let animals = vec!["Rabbit", "Dog", "Cat"];
    // make sure you use .iter() with vectors so that the ownership does not change
    for a in animals.iter() {
        println!("The number is {}", a)
    }

    // to find the index of the value in the array
    let cars = vec!["volks", "ferrari", "porshe"];

    for (index, a) in cars.iter().enumerate() {
        println!("The cars name is {},{}", a, index)
    }

    // a function that prints numbers to 10
    print_numbers_to(10);

    // mutable varibles returns an empty string
    let mut name = String::new();

    let greeting = "Enter your name: ";
    println!("===>,{greeting}");

    io::stdin()
        .read_line(&mut name)
        .expect("Didn't receive input");
    println!("Hello,{name}");

    let use_struct_impl =  Rectangle{width:10,height:90};
    use_struct_impl.print_vals()

}

fn print_numbers_to(mut num: u32) {
    for n in 1..num {
        println!("{}", n)
    }
}

// Checks if a number is even or not
fn is_even(mut num: u32) -> bool {
    if (num % 2 == 0) {
        return true;
    }
    return false;
}
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Rectangle{
    width:u8,
    height:u8
}

impl Rectangle{
    fn print_vals(&self){
        println!("{}{}",self.width,self.height)
    }
}