# Rust Collections Summary Notes

## Vectors (`Vec<T>`)

### Creating Vectors
- Create empty vector: `let v: Vec<i32> = Vec::new();`
- Create with initial values: `let v = vec![1, 2, 3];` (using `vec!` macro)

### Modifying Vectors
- Add elements: `v.push(value);` (requires `mut` vector)
- Remove elements: `v.pop()` removes and returns the last element

### Accessing Vector Elements
- Index syntax: `&v[2]` (returns reference, panics if out of bounds)
- `get` method: `v.get(2)` (returns `Option<&T>`, returns `None` if out of bounds)

### Iterating Over Vectors
- Read-only iteration: `for i in &v { println!("{i}"); }`
- Mutable iteration: `for i in &mut v { *i += 50; }`

### Important Vector Rules
- Vectors can only store elements of the same type
- When adding elements, vector might need to reallocate memory
- Cannot hold a reference to an element and modify the vector simultaneously
- When vector is dropped, all its elements are dropped too

### Storing Multiple Types
- Use enum variants to store different types in the same vector:
```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

## Strings

### Types of Strings
- String slice (`&str`): immutable reference to UTF-8 encoded string data
- `String`: growable, mutable, owned string type (wrapper around `Vec<u8>`)

### Creating Strings
- Create empty: `let mut s = String::new();`
- From string literal: `let s = "initial contents".to_string();`
- Using `from`: `let s = String::from("initial contents");`

### Modifying Strings
- Append string slice: `s.push_str("bar");`
- Append single character: `s.push('l');`
- Concatenation:
  - Using `+` operator: `let s3 = s1 + &s2;` (note: `s1` is moved)
  - Using `format!` macro: `let s = format!("{s1}-{s2}-{s3}");` (doesn't take ownership)

### String Complexities
- Indexing not supported: `let h = s1[0];` will not compile
- Reason: UTF-8 encoding means characters can be multiple bytes
- String slicing requires caution: `&hello[0..4]` works but could panic if slicing mid-character

### String Iteration
- By characters: `for c in "Зд".chars() { println!("{c}"); }`
- By bytes: `for b in "Зд".bytes() { println!("{b}"); }`

### String Representation
- `String` is a wrapper over `Vec<u8>`
- UTF-8 encoded characters may use 1-4 bytes each
- Three ways to view strings:
  - As bytes
  - As scalar values (chars)
  - As grapheme clusters (user-perceived "letters")


## Hash Maps (`HashMap<K, V>`)

### Creating Hash Maps
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

### Accessing Values
```rust
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

### Iterating Over Hash Maps
```rust
for (key, value) in &scores {
    println!("{key}: {value}");
}
```

### Ownership Rules
- Types implementing `Copy` trait (like `i32`) are copied into the hash map
- Owned values (like `String`) are moved, and the hash map becomes the owner
- If references are inserted, they must remain valid for at least as long as the hash map

### Updating Hash Maps
1. Overwriting a value:
```rust
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25); // Value is now 25
```

2. Only inserting if key doesn't exist:
```rust
scores.entry(String::from("Yellow")).or_insert(50);
```

3. Updating based on old value:
```rust
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

### Hashing Function
- Default: SipHash (provides DoS attack resistance)
- Can be customized by implementing the `BuildHasher` trait