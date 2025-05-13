

fn main(){
    let mut s = String::from("Hello, world");
    //mutable reference

    //let p = s[0..1];
    //not allowed, bc unsized so can't be on stack
    let m = &s[0..1];
    //works bc use of reference, a pointer can be on stack

    let word = first_word(&s);
    //immutable reference, if tried to make mutable it would throw error

    println!("{word}");
    //immutable use here

    s.clear();
    //mutable use here, should be fine as long as you dont use word again

    println!("{}", s.len());



    //println!("{word}")
    //throws error

}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i]
        }
    }
    return &s[..]
}