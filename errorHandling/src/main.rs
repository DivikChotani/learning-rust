use std::{fs::{self, File}, io::{Read, Seek, SeekFrom, Write}};

fn create_file(name: &str) -> File {
    let mut file =  File::create_new(name).expect("FAILED");
    file
}
fn main() {
    println!("Hello, world!");

    //panic!("panic macro");
    //panic exits with non 0 return value

    let mut v = vec![1,2,3];

    let t = &mut v[0];
    *t = 99;


    println!("{v:#?}");

    //re emphasizing how to change element in array

    //cargo run with the --release to remove debug symbols
    let file = File::options()
        .read(true)
        .write(true)
        .append(true)
        .open("./hello.txt");
    let mut greeting = match file {
        Ok(file) => file,
        Err(_) => create_file("./hello.txt")
    };

    //you have to put the pattern first then what you compare it to
    if let Ok(_) = greeting.write(b"hello kite and barnada"){
        println!("SUCCESS")
    }
    else {
        println!("FAILED TO WRITE")
    };
   

    greeting.seek(SeekFrom::Start(0)).expect("FAILED");

    let mut stuff = String::new();
    let Ok(_) =  greeting.read_to_string(&mut stuff) else{
        fs::remove_file("./hello.txt").expect("FAILED");
        panic!("ERROR")
    };
    println!("{stuff}");
    fs::remove_file("./hello.txt").expect("FAILE");

}
