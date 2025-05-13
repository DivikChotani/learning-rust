
# Understanding Rust Reference Binding Patterns

Starting with: `let mut s = String::from("hello");`

## The Four Reference Binding Patterns

### 1. `let r1 = &s;` - Immutable Reference
- Creates an **immutable reference** to `s`
- The binding `r1` is immutable (cannot be reassigned)
- You can **read** the data in `s` through `r1`, but cannot modify it
- Multiple immutable references are allowed simultaneously

```rust
let r1 = &s;
println!("{r1}"); // ✅ Reading is allowed
// r1.push_str(" world"); // ❌ Cannot modify through immutable reference
// r1 = &another_string; // ❌ Cannot reassign immutable binding
```

### 2. `let r3 = &mut s;` - Mutable Reference
- Creates a **mutable reference** to `s`
- The binding `r3` is immutable (cannot be reassigned)
- Allows **modifying** the data in `s` through `r3`
- **Exclusive access**: no other references (mutable or immutable) can exist simultaneously

```rust
let r3 = &mut s;
r3.push_str(" world"); // ✅ Modification allowed
// r3 = &mut another_string; // ❌ Cannot reassign immutable binding
```

### 3. `let mut r2 = &mut s;` - Mutable Reference with Mutable Binding
- Creates a **mutable reference** to `s`
- The binding `r2` is mutable (can be reassigned)
- Allows **modifying** the data in `s` through `r2`
- **Exclusive access**: no other references (mutable or immutable) can exist simultaneously
- Can reassign `r2` to point to another mutable reference

```rust
let mut r2 = &mut s;
r2.push_str("!"); // ✅ Modification allowed
let mut s2 = String::from("another");
r2 = &mut s2; // ✅ Reassignment allowed
```

### 4. `let mut r4 = &s;` - Immutable Reference with Mutable Binding
- Creates an **immutable reference** to `s`
- The binding `r4` is mutable (can be reassigned)
- You can **read** the data in `s` through `r4`, but cannot modify it
- Can reassign `r4` to point to another immutable reference

```rust
let mut r4 = &s;
println!("{r4}"); // ✅ Reading allowed
// r4.push_str(" world"); // ❌ Cannot modify through immutable reference
let s2 = String::from("world");
r4 = &s2; // ✅ Reassignment allowed
```

## Compatibility Table

| Pattern | Reference Type | Binding Mutable? | Can Modify Data? | Can Reassign Variable? | Notes |
|---------|---------------|-----------------|-----------------|----------------------|-------|
| `let r1 = &s;` | `&String` | No | ❌ No | ❌ No | Read-only reference |
| `let r3 = &mut s;` | `&mut String` | No | ✅ Yes | ❌ No | Exclusive mutable access |
| `let mut r2 = &mut s;` | `&mut String` | Yes | ✅ Yes | ✅ Yes | Can reassign to another `&mut` |
| `let mut r4 = &s;` | `&String` | Yes | ❌ No | ✅ Yes | Still read-only for data |

## Key Takeaways

1. **Reference Mutability** (`&` vs `&mut`) determines whether you can modify the referenced value
2. **Binding Mutability** (`let` vs `let mut`) determines whether you can reassign the variable

## Common Errors and Rules

1. **Mutable references are exclusive**:
   ```rust
   let r1 = &s;
   let r3 = &mut s; // ❌ Error: cannot borrow `s` as mutable because it is also borrowed as immutable
   ```

2. **No multiple mutable references**:
   ```rust
   let r3 = &mut s;
   let r2 = &mut s; // ❌ Error: cannot borrow `s` as mutable more than once at a time
   ```

3. **References must be valid for their entire lifetime**:
   ```rust
   let r1;
   {
       let s2 = String::from("temp");
       r1 = &s2; // ❌ Error: `s2` does not live long enough
   } // s2 is dropped here while still borrowed
   println!("{r1}");
   ```

4. The "**non-lexical lifetimes**" feature allows references to be valid only until their last usage:
   ```rust
   let r1 = &s;
   println!("{r1}"); // r1 is no longer used after this point
   let r3 = &mut s; // ✅ This works in modern Rust because r1's scope effectively ends above
   r3.push_str(" world");
   ```