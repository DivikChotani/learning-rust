//const are never mutable and must always be annotate and compiler recommends capitalizing them
//set main to default run with directory name if using cargo for convenience
const TV: u32 = 888;
fn main() {
    // let x = 5; won't let you reassign value
    let mut x = 5;
    {
    let x = x+1;
    // but if i had not written let, it would have changed the value of the outer scope x
    //doesn't work because x is default immutable
    println!("The value of x is: {x} in the inner scope");
    }
    println!("The value of x is: {x} in the outer scope");
    let spaces = "   ";
    let spaces = spaces.len();
    //shadowing a variable
    println!("{spaces}")
}
