use std::io;

fn main() {
   println!("Enter the number you want me to guess");


   let mut guess = String::new();


   io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");




   let guess_int:i32 = guess.trim().parse().unwrap();


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
    println!("{}", min)
   }
   println!("You guessed: {}", guess);
}



