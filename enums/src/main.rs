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

    handleIP(version4);
    handleIP(version6);
    handleIP(noip);

}


//handling enums
fn handleIP(ipAdd : IP){
    match ipAdd {
        IP::V4(v4) => println!("ip is v4 with {} {} {}", v4.0, v4.1, v4.2),
        IP::V6(v6) => println!("ip is v6 with {}", v6.addr),
        IP::No(_) => println!("No ip"),
    };
}
