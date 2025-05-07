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

    //References and Borrowing, immutable by default
    let mut s1 = String::from("Hairy");


    let len = calculate_len(&s1); // passing by reference

    println!("Len of {s1} is {len}");

    let temp = &mut s1;
    // After this point, s1 and temp cannot be used at the same time.
    // However, once temp is no longer used, s1 becomes usable again.

    passing_mut_ref(temp);

    println!("{s1}");

    {
        let r1 = &mut s; 
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    //println!("{temp}");

    let mut hello = String::from("hello");

    let hellor1 = &s; // no problem
    let hellor2 = &s; // no problem
    println!("{hellor1} and {hellor2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

}

fn takes_ownership(s : String){
    println!("ownesrhip of s: {s} transferred here, and will go out of scope after this");
}

fn makes_copy(t: i32){
    println!("copy of variable t: {t}, not same reference");
} 
//Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.


fn calculate_len(refs : &String) -> usize {
    return refs.len();
} //refs does not have ownership of the string, so the value is not dropped


fn passing_mut_ref(t: & mut String) {
    t.push_str("Brandan");
}