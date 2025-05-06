//seperate, non module binaries must be in the bin folder
use std::io;


fn main() {
    println!("hi");

    //expects a description of the type of variable to parse it to
    let guess: u32 = "42".parse().expect("Not a number!");

    //can use _ as visual seperator
    let comma = 1_000;
    println!("{comma}");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    //basically no reason nowadays for f32; f64 is basically same speec on modeern cpus and more precise

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder just like c
    let _remainder = 43 % 5;
    //underscore to tell compiler it's intentionally unused
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
    //note every char is 4, not 1 byte

    // tuple, can be mutable and can infer data type

    let tup : (i32, f64, u8) = (512, 8.0, 1);

    let (_, y, _) = tup;
    println!("Y is {y}");

    //same as accesing like tup.1
    let f : bool = y == tup.1;
    println!("{f}");
    let mut tupp  = ("HELLO", 'o');

    //modify field
    tupp.1 = 'k';
    let (a, b) = tupp;
    println!("{a} {b}"); 


    //simple array code 
    let t: bool = true;

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index : usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("some error");
            0
        }
    };
    let element = a[index];
    println!("{element}");

}

