fn main() {
    let rect1 = Rectangle {
        width: dbg!(30), //dbg returns ownership of val, so this is fine
        height: 50,
        name: dbg!(String::from("HI")) // same here
    };
    let a = area(&rect1);
    println!("{a}");

    let _b = dbg!(5);
    //Yes, all primitive types and nearly all standard library types in Rust implement the Debug


    //uses the derive(Debug) feature, #? for pretty-print, ? for normal
    println!("rect1 is {rect1:#?}");


}

fn area(r: &Rectangle) -> i32 {
    r.height * r.width
}
#[derive(Debug)] //attribute we derived and gave to rectange
struct Rectangle {
    height: i32,
    width: i32,
    name: String
}