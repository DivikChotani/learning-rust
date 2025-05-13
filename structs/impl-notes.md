# Rust Methods vs Associated Functions

## Definitions

### Associated Functions

- Does **not** take `self`, `&self`, or `&mut self`
- Called with `TypeName::function_name()`
- Example: `fn return_true() -> bool`

### Methods

- Takes `self`, `&self`, or `&mut self`
- Called on an instance: `instance.method()`
- Example: `fn area(&self) -> i32`

## Key Rules for Implementation Blocks

Inside an `impl` block:

1. You can only call another method **after it's already been defined above** in the same block.

2. **Associated functions** (not reliant on self) **can be called from anywhere**, regardless of where they're defined in the block.

