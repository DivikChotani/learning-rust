fn main() {
    let rect1 = Rectangle {
        width: dbg!(30), //dbg returns ownership of val, so this is fine
        height: 50,
        name: dbg!(String::from("HI")) // same here
    };
    let a = area(&rect1);
    println!("{a}");

    let mut b = dbg!(5);
    //Yes, all primitive types and nearly all standard library types in Rust implement the Debug


    //uses the derive(Debug) feature, #? for pretty-print, ? for normal
    println!("rect1 is {rect1:#?}");

    //Use of IMPL from here on

    b = rect1.area();

    let t = Rectangle::new(3, 4);
    println!("is t bigger than rect1? {}", t.is_bigger(&rect1));

    println!("True for no reason: {}", Rectangle::return_true())

    //If a function in an impl block does not take self, &self, or &mut self, 
    // then it’s an associated function and must be called with ::, like
    // TypeName::function_name() NOT THE NAME OF THE INSTANCE
    // Rectangle::return_true() not t::return_true


}

fn area(r: &Rectangle) -> i32 {
    r.height * r.width //similar to classes in c++, but inside an impl block
}
#[derive(Debug)] //attribute we derived and gave to rectange
struct Rectangle {
    height: i32,
    width: i32,
    name: String
}


impl Rectangle {
    fn new(h: i32, w: i32) -> Self {
        Self { height: h, width: w, name: String::from("HI") }
    }
    fn area(&self) -> i32 {
        println!("{}", Rectangle::return_true());

        self.height * self.width //similar to classes in c++, but inside an impl block
        //even if you wanted to, you cant call is bigger here, not yet in scope

    }
    fn is_bigger(&self, other_rect: &Rectangle) -> bool{
        return self.area() >= other_rect.area()
        // can call impl functions from impl, only if function is defined before and gets in scope,
    }

    fn return_true() -> bool{
        true //not related, an associated function
    }
}