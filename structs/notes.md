# Rust Tuple Structs and Unit Structs: Use Cases and Comparisons

## Tuple Structs

### What is a Tuple Struct?
A tuple struct is a named tuple that defines a new unique type in Rust.

```rust
struct Color(i32, i32, i32);  // A tuple struct
```

### Key Differences: Tuple Struct vs. Regular Tuple

| Feature | Tuple Struct (`Color(i32, i32, i32)`) | Tuple (`(i32, i32, i32)`) |
|---------|--------------------------------------|---------------------------|
| Has a name | ✅ Yes (Color) | ❌ No — anonymous type |
| Defines a new type | ✅ Yes — Color is its own type | ❌ No — just (i32, i32, i32) |
| Type-safe distinction | ✅ Yes (not interchangeable with other structs or tuples) | ❌ No |
| Field access | ✅ With dot syntax: c.0, c.1 | ✅ Same |
| Traits needed for print/debug | ✅ Needs #[derive(Debug)] manually | ✅ Already printable (if types are) |
| Equality comparison | ✅ Needs #[derive(PartialEq)] | ✅ Automatically implemented |

### Example Usage

```rust
struct Color(i32, i32, i32); // tuple struct

fn main() {
    let c = Color(255, 0, 0);       // This is a `Color`, not a tuple
    let t = (255, 0, 0);            // Regular tuple

    println!("c.0 = {}", c.0);      // Accessing fields works the same way
    println!("t.0 = {}", t.0);      

    // Type-safe distinction:
    // let c2: (i32, i32, i32) = c; // Error: mismatched types
}
```

### Why Use Tuple Structs?

Tuple structs are useful when you:

1. **Need type distinction**: Create distinct types that aren't interchangeable
2. **Don't need named fields**: When the order is obvious or conventional
3. **Want minimal syntax**: Less verbose than full structs with named fields
4. **Prevent logical errors**: Type safety prevents mixing similar data

### Common Use Cases

```rust
struct Point(f64, f64);     // 2D coordinates
struct Millimeters(u32);    // Newtype pattern for units
struct Seconds(u32);        // Another unit-specific type
struct RGB(u8, u8, u8);     // Color representation
```

This prevents mixing `Seconds(5)` and `Millimeters(5)` accidentally, even though both contain a `u32`.

### Comparing Tuple Structs

When working with tuple structs and equality:

```rust
// Won't compile without deriving PartialEq
let t1 = Color(2, 3, 4);
let t2 = Color(2, 3, 4);
// println!("{}", t1 == t2); // Error

// Fix by adding:
#[derive(PartialEq)]
struct Color(i32, i32, i32);
```

With tuple structs, you must explicitly derive equality traits, unlike regular tuples which implement them automatically.

## Unit Structs

### What is a Unit Struct?

A unit struct is a struct with no fields - it contains no data but defines a distinct type.

```rust
struct AlwaysEqual;  // A unit struct
```

### Properties and Use Cases

| Feature | Unit Struct |
|---------|-------------|
| Memory usage | Zero-sized (no runtime storage) |
| Instantiating | Just write the name: `let x = AlwaysEqual;` |
| Equality comparison | Requires #[derive(PartialEq)] to compare |
| Typical uses | Marker types, type-level programming, phantom types |

### Why Use Unit Structs?

Unit structs are useful when:

1. **Creating marker types**: For traits or type-level logic
2. **Zero-cost abstraction**: Type safety with no runtime overhead
3. **Type state pattern**: Encode state in the type system
4. **Trait implementations**: When you need a concrete type to implement a trait

### Unit Struct vs. Empty Enum

| Purpose | Unit Struct `struct AlwaysEqual;` | Single-Variant Enum `enum Variant { Single }` |
|---------|-----------------------------------|----------------------------------------------|
| Has a value? | ✅ Yes (AlwaysEqual) | ✅ Yes (Variant::Single) |
| Zero-sized? | ✅ Yes | ✅ Yes |
| Best for marker types? | ✅ Yes, more concise | ⚠️ Possible, but more verbose |
| Construction syntax | Direct: `let x = AlwaysEqual;` | With variant: `let x = Variant::Single;` |

### Example Usage of Unit Structs

```rust
// As marker types for traits
struct Marker;
impl SomeTrait for Marker { /* ... */ }

// Zero-cost configuration
struct DebugMode;
struct ReleaseMode;

fn do_thing<T: Mode>() { /* ... */ }

// Type state pattern
struct Locked;
struct Unlocked;

struct Door<State> {
    // Fields...
    state: State,
}

// With phantom data for generics
use std::marker::PhantomData;

struct Wrapper<T> {
    _marker: PhantomData<T>,
}
```

### Making Unit Structs Comparable

```rust
#[derive(PartialEq, Eq)]
struct AlwaysEqual;

fn main() {
    let b = AlwaysEqual;
    println!("is b always equal? {}", b == AlwaysEqual); // prints "true"
}
```

By deriving PartialEq, you enable equality comparison. Since there are no fields, all instances of AlwaysEqual are considered equal.

## Summary

- **Tuple structs** provide named types with positional access and type safety, ideal when field names aren't necessary
- **Unit structs** define zero-sized marker types, perfect for type-level programming and compile-time enforcement
- Both require explicit trait derivation for operations like comparison and debugging
- Both play important roles in Rust's type system for creating safer, more expressive code