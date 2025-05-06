# 🦀 Rust Control Flow Expression Examples

This document shows how Rust handles control flow structures like blocks, `if`, `loop`, `match`, and others as expressions or statements.

---

## 1. `{}` Block as an Expression

```rust
fn block_expression() {
    let x = {
        let a = 5;
        let b = 10;
        a + b // ✅ No semicolon — value returned
    };

    println!("Block result: {x}"); // Prints: 15
}
```

```rust
fn block_unit() {
    let x = {
        let a = 5;
        let b = 10;
        a + b; // ❌ Semicolon discards value
    };

    println!("Block result: {:?}", x); // Prints: ()
}
```

---

## 2. `if / else` as an Expression

```rust
fn if_expression() {
    let cond = true;
    let result = if cond {
        42 // ✅ Returned if cond is true
    } else {
        24
    };

    println!("If result: {result}"); // Prints: 42
}
```

```rust
fn if_unit() {
    let cond = true;
    let result = if cond {
        42; // ❌ Semicolon — this branch returns ()
    } else {
        24
    };

    println!("If result: {:?}", result); // ⚠️ Prints: ()
}
```

---

## 3. `match` as an Expression

```rust
fn match_expression() {
    let num = 2;

    let desc = match num {
        1 => "one",
        2 => "two",
        _ => "other",
    };

    println!("Matched: {desc}"); // Prints: two
}
```

```rust
fn match_unit() {
    let num = 2;

    let desc = match num {
        1 => { "one"; },     // ❌ semicolon → returns ()
        2 => { "two" },      // ✅ no semicolon → returns "two"
        _ => { "other" },
    };

    println!("Matched: {:?}", desc); // ⚠️ Will be () if selected arm has semicolon
}
```

---

## 4. `loop` as an Expression

```rust
fn loop_expression() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter * 2; // ✅ break with value
        }
    };

    println!("Loop result: {result}"); // Prints: 10
}
```

---

## 5. `while` is Not an Expression

```rust
fn while_statement() {
    let mut i = 0;

    while i < 3 {
        println!("i = {i}");
        i += 1;
    }

    // let x = while i < 3 { i += 1 }; // ❌ ERROR: `while` cannot be used in expression position
}
```

---

## 6. `for` is Not an Expression

```rust
fn for_statement() {
    for i in 1..4 {
        println!("i = {i}");
    }

    // let x = for i in 1..4 { println!("{i}"); }; // ❌ ERROR: `for` cannot be used in expression position
}
```
