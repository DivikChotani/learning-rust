fn main() {
    println!("Hello, world!");
    {                      // s is not valid here, it’s not yet declared
        let mut s = "hello";   // s is valid from this point forward
        //gives warning because modified before use
        //s is of type &'static str — string slice, borrowed from a string literal.
        //Immutable and fixed in size, in read only binary data
        s = "world"; //reassigns s to another &str
        println!("{s}")

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    let mut s = String::from("hello");
    s.push_str(" world"); //change on the heap
    //type string, Heap allocation

    let _p = s; //only copies reference, makes the original reference s invalid
    s = String::from("new");
    //de allocates the hello world

    let mut pp = s.clone() ;//deep copy

    pp.push_str("t");

    let x = 1;

    let mut y = x; //deep copy bc allocated on stack
    y+=1;

    let st = String::from("gonna die");

    takes_ownership(st);

    makes_copy(y);
    println!("x still valid: {x}");

}

fn takes_ownership(s : String){
    println!("ownesrhip of s: {s} transferred here, and will go out of scope after this");
}

fn makes_copy(t: i32){
    println!("copy of variable t: {t}, not same reference");
} //Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

