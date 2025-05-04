use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
   println!("Enter the number you want me to guess");


   let mut guess = String::new();


   io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");




   let guess_int:i32 = guess.trim().parse().expect("WRONG");


   let mut min:i32 = -500000;
   let mut max:i32 = 500000;




   while min < max{
       let guess:i32 = min + (max - min) / 2;
       println!("{}",guess);
       if guess > guess_int{
           max = guess - 1;
       }
       else if guess < guess_int{
           min = guess + 1;
       }
       else{
           break;
       }
   }
   if max == min{
    println!("{}",min)
   }
   println!("Found it! {guess}");



   println!("You guess between 1 and 100");

   let secret_number = rand::thread_rng().gen_range(1..=100);

   loop {
    
        let mut guess_2 = String::new();
        println!("Write your guess");
        io::stdin()
            .read_line(&mut guess_2)
            .expect("Failed to read line");

        let guess_2: i32 = match guess_2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess_2}");

        match guess_2.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break
                },
        }


   }
}



