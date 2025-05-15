use std::{cell, io};

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("Hello, world!");
    let mut v = vec![1, 2, 3, 4, 5];

    //let dne = &v[100];
    //crashes compile time if v is too small

    let dne = v.get(100);
    //sends an option
    let mut value = match dne {
        Some(x) => x,
        None => v.get(0).unwrap(),
    };

    println!("value: {value}");

    v.insert(3, *value);
    //have to dereference
    let mut q = 0;
    for i in &mut v{
        *i +=50; // i is a reference to the value, change the value at the address of i
        q = *i // copies the value at the address
    }
    q = 0; //wont change v,

    println!("{v:#?}");

    let mut cells: Vec<SpreadsheetCell> = Vec::new();

    println!("enter the number");

    let mut val = SpreadsheetCell::Int(3);
    cells.push(val);
    let mut q = String::new();
    io::stdin()
        .read_line(&mut q)
        .expect("Wierd");

    if q.trim().eq("1") {
        cells.push(SpreadsheetCell::Float(2.5));
    }
    else {
        cells.push(SpreadsheetCell::Int(2));
    }

    println!("{cells:#?}");

    //STRINGS

    let mut s1 = String::from("value");
    s1.push('1'); //push adds a single char

    s1.push_str("diddy"); //push str takes &str, (strings can also go, deref coersion)

    let q = String::from("HI");
    s1.push_str(&q);// but must pass reference to string, so q will still have ownership

    println!("{s1} {q}");

    let mut s2 = s1 + &q; // addition takes string and &str, so s1 looses access here

    //s1.clear(); will throw error because s1 not valid anymore
    s2.clear(); //this is allowed

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; //works left to right, "-" is a string literal, so it
    //is already a &str

    println!("{s}");

    //whenever using [], must use &
    //let mut k = s[0..1]; not allowed
    let p = &s[0..1];
    println!("{p}");

    //to get nth char
    let t = s.chars().nth(2); //nth is not O(1)
    //if let statement
    if let Some(ch) = t {
        println!("t is a valid index, pointing to {ch}"); //use ch, because that is the unwrapped char provided if the if let succeeds
        // ch = 'q';
        // println!("{s}"); won't modify the array, nth returns a copy
    }
    else{
        println!("FAILED");
    }

    for b in "Зд".bytes() {
        println!("{b}"); //this will print the byte values, each char is two bytes here
        //so it will print two bytes
    }



}
