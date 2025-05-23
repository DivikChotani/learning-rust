fn main() {
   //println!("Hello, world!");

   //create user,need mut to be mutable
   let mut u1 = User {
    active: true,
    username: String::from("u1"),
    email: String::from("u1@gmail"),
    sign_in_count: 1
   };
   u1.active = ! u1.active;

   println!("User 1: Name: {} emai: {}", u1.username, u1.email);

   //build a user
   let m = build_user(None, None);

   println!("m: active: {}, name: {}, email: {}, sign in count: {}", m.active, m.username, m.email, m.sign_in_count);
   let u2 = User {
    email: String::from("u2@gmail"),
    ..u1 // saying copy everything else from u1
   };
   //but this makes u1 unusable because u1 was allocated on the heap, except for u1.email
   println!("{}", u1.email);
   println!("{}", u2.username);


   //println!("{}", u1.username);
   //this thows an error
   let mut l = (2,3, 4);

   let mut k = (2,3,4);

   let t = Color(2,3,4);

   let m: Color = Color(2,3, 4);

   println!("{}", l == k);
   //this is fine, as long as tuple has same types inside defined and they define equality

   //can not do this, must define equality
   //println!("{}", t == m);

   let b = AlwaysEqual;
   //has some use cases


   
}

fn build_user(email: Option<&str>, username: Option<&str>) -> User{
    let username = username.unwrap_or("default name").to_string();
    let email = email.unwrap_or("default@gmail").to_string();
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }

    
}
struct Color(i32, i32, i32);//a tuple struct

struct AlwaysEqual;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64
}
