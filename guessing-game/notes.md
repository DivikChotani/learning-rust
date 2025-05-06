# Rust Guessing Game – Comprehensive Summary

## Project Setup
* Create a new project:

```bash
cargo new guessing_game
cd guessing_game
```

* Run with:

```bash
cargo run
```

* Project structure:
  - `src/main.rs` - Main source code file
  - `Cargo.toml` - Project configuration and dependencies

## User Input
* Import the input/output library:
```rust
use std::io;
```

* Read and store a user's guess:

```rust
let mut guess = String::new();
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

* Print using placeholder syntax:

```rust
println!("You guessed: {guess}"); // Modern Rust syntax
// Or
println!("You guessed: {}", guess); // Traditional syntax
```

## Random Number Generation
* Add dependency to `Cargo.toml`:
```toml
[dependencies]
rand = "0.8.5"
```

* Import and use:

```rust
use rand::Rng; // Brings Rng trait into scope
let secret_number = rand::thread_rng().gen_range(1..=100);
```

* Understanding what's happening:
  - `thread_rng()` - Gets the random number generator local to the current thread
  - `gen_range(1..=100)` - Generates a number in the inclusive range 1-100

## Converting and Comparing Input
* Convert string to number with error handling:

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

* Compare with `secret_number` using:

```rust
use std::cmp::Ordering;

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

## Loop for Multiple Guesses
* Wrap guessing logic in an infinite loop to allow repeated tries:
```rust
loop {
    // Guessing logic here
    
    // Exit condition
    if guess == secret_number {
        println!("You win!");
        break;
    }
}
```

* Add `break;` when guess is correct (typically in the `Ordering::Equal` arm of the match)

## Error Handling for Invalid Input
* Use `match` for robust error handling to skip bad input:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Please enter a valid number!");
        continue; // Skip to next iteration
    }
};
```

## Final Behavior
* Random number is generated and hidden from the user
* User is prompted repeatedly until they guess correctly
* Provides feedback on each guess (too high, too low)
* Handles invalid input gracefully without crashing
* Ends with congratulatory message when correct

## Concepts Practiced

### Variable Handling
* **Variables with `let`**: Creating named storage
* **Mutability with `mut`**: Allowing values to change
  ```rust
  let mut guess = String::new(); // Mutable variable
  let secret_number = 42; // Immutable variable
  ```

### Shadowing
* Reusing variable names for different values or types:
  ```rust
  let guess = String::new(); // String type
  let guess: u32 = guess.trim().parse().expect("Not a number"); // u32 type
  ```
* Differs from mutation as it creates a new variable with same name

### Input/Output with `io`
* Standard I/O handling from the standard library
* `stdin()` - Gets a handle to terminal input
* `read_line()` - Reads user input until newline
* `println!` - Macro to print formatted text to console

### External Crates
* **Using `rand`**: Including external functionality
* **Cargo dependency management**: Version specification in `Cargo.toml`
* **Using external traits**: Bringing `Rng` into scope

### Types and Type Conversion
* **String handling**: Working with text input
* **Numeric types**: Using `u32` for unsigned 32-bit integers
* **Parsing**: Converting between string and numeric types
* **Method chaining**: `guess.trim().parse()`

### Pattern Matching
* **`match` expressions**: Exhaustive pattern matching
* **`Ordering` enum**: Using predefined comparison results:
  ```rust
  match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => println!("You win!"),
  }
  ```

### Control Flow
* **Looping with `loop`**: Creating infinite loops
* **`break`**: Exiting loops
* **`continue`**: Skipping to next iteration

### Error Handling
* **`Result` enum**: Handling operations that can fail
* **`expect()`**: Unwrapping values or crashing with message
* **Pattern matching errors**:
  ```rust
  match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue, // Graceful error handling
  }
  ```
* **Using the `_` wildcard**: Matching any error without specifying

### References and Borrowing
* **Using `&` for references**: Passing values without taking ownership
  ```rust
  guess.cmp(&secret_number) // & creates reference to secret_number
  ```

### Methods and Function Calls
* **Calling methods on types**: `thread_rng().gen_range(1..=100)`
* **Chaining method calls**: `guess.trim().parse()`

### Ranges
* **Using the range syntax**: `1..=100` for inclusive ranges