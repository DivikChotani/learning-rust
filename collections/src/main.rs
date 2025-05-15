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

    println!("{cells:#?}")

}
