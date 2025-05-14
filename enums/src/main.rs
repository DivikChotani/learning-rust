use std::option;

//creating enums, camel case recommended
enum IpAddKind {
    V4,
    V6,
}

// tuple struct (fields have no names, only types)
struct IpV4 (i32, i32, i32);

//normal struct
struct IpV6 {
    addr: String,
}

//unit struct
struct NoIp;

//enums can also store values in the enumerations
enum IP {
    V4(IpV4),
    V6(IpV6),
    No(NoIp),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}


enum Coin {
    penny,
    nickel,
    dime,
    quarter(UsState), //taking an enumerator as a parameter to an enum
}

fn main() {
    println!("Hello, world!");
    let m = IpAddKind::V4;


    //matches are switch statements
    match m {
        IpAddKind::V4 => println!("M is v4"),
        IpAddKind::V6 => println!("M is v6"),
    };

    //enums have a provided constructor, just pass in the value the enum requires
    let version4 = IP::V4(IpV4(1,2,3));

    let v6ip = IpV6 {
        addr : String::from("HI")
    };

    let version6 = IP::V6(v6ip);

    // construct the struct and wrap it in the enum variant   
    let noip = IP::No(NoIp);

    handle_ip(version4);
    handle_ip(version6);
    handle_ip(noip);
    print_coin(Coin::quarter(UsState::California));
    print_coin(Coin::dime);


}


//handling enums
fn handle_ip(ipAdd : IP){
    match ipAdd {
        IP::V4(v4) => println!("ip is v4 with {} {} {}", v4.0, v4.1, v4.2),
        IP::V6(v6) => println!("ip is v6 with {}", v6.addr),
        IP::No(_) => println!("No ip"),
    };
}

//using options, have a none, and some case, depending on the variable inside option
fn add_one(x: Option<i32>) -> Option<i32> {
    match  x{
        None => None,
        Some(x) => Some(x+1), //must be exhaustive, can't have a missing case
    }
}

fn print_coin(coin : Coin) -> i32{
    match coin {
        Coin::penny => {
            println!("you have one cent");
            1
        }
        Coin::quarter(state) => {
            println!("You have a quarter from {state:#?}");
            25
        }
        _ => {
            println!("you have either a dime or nickel, im guessing dime");
            10
        } //can have a catch all case here using underscore _
    }
}
