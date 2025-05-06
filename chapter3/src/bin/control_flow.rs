use std::io;
fn main(){
    let mut x = String::new();
    println!("give me x");
    io::stdin()
        .read_line(&mut x)
        .expect("FAILED");
    let x:i32 = x.trim().parse().expect("failed");

    //if statement block expression
    let t = {
        if x % 3 == 0 {
            x * 2
        }
        else {
            1
        }
    };

    //can also be written as
    let condition = x % 3 == 0; // can also write x % 3 == 0 in the statement directly
    let q = if condition {x * 2} else {1};
    //one above makes more sense because it's explicitly using if statements to return


    //if and else arms must have same type
    println!("{t}");
    println!("{q}");

    let mut count = 0;
    loop {
        if count == 5 {
            break
        };
        println!("HAY");
        count +=1;
    };

    //loops can be set to expressions

    count = 0;
    let x = loop {
        if count == 5 {
            break "abcd"
        }
        count +=1
    };
    println!("{x}");


    let a = [10, 20, 30, 40, 50];

    //iterate through all elements
    for num in a {
        println!("{num}");
    }

    //get index until length of loop
    for num in (0..a.len()).rev(){
        println!("{ }", a[num]);

    }
}