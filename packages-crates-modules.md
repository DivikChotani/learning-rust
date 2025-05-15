# Rust Notes: Packages, Crates, and Modules

## Packages & Crates

* **Crate**: Smallest unit of code the Rust compiler considers. Can be:

  * **Binary crate**: Contains a `main` function; compiles to an executable.
  * **Library crate**: Does not have a `main`; provides reusable code.
* **Crate root**:

  * Binary: `src/main.rs`
  * Library: `src/lib.rs`
* **Package**:

  * Contains 1+ crates (only one library crate allowed).
  * Always includes a `Cargo.toml`.
  * Convention:

    * Binary crate: `src/main.rs`
    * Library crate: `src/lib.rs`
    * Multiple binaries: `src/bin/*.rs`

## Modules & Privacy

* **Modules**: Group related functionality. Declared with `mod`.
* **Default**: Items are private to their parent module.
* Use `pub` to expose modules or items:

  ```rust
  pub mod hosting {
      pub fn add_to_waitlist() {}
  }
  ```
* **Module hierarchy** (tree-like):

  ```text
  crate
  └── front_of_house
      ├── hosting
      └── serving
  ```

## Paths

* **Absolute path**: Starts from crate root.

  ```rust
  crate::front_of_house::hosting::add_to_waitlist();
  ```
* **Relative path**: Starts from current module.

  ```rust
  front_of_house::hosting::add_to_waitlist();
  ```
* Use `super` to refer to parent module.

## `use` Keyword

* Simplifies long paths:

  ```rust
  use crate::front_of_house::hosting;
  hosting::add_to_waitlist();
  ```
* Scope-sensitive: must use `use` within the same scope.
* **Idioms**:

  * Functions: use parent module
  * Structs/enums: bring item directly

    ```rust
    use std::collections::HashMap;
    ```
* Aliases:

  ```rust
  use std::io::Result as IoResult;
  ```
* **Re-export** with `pub use`:

  ```rust
  pub use crate::front_of_house::hosting;
  ```

## Structs & Enums Visibility

* `pub struct`: Fields are private by default; make individual fields `pub`.
* `pub enum`: Variants are public by default.

## External Crates

* Add to `Cargo.toml`:

  ```toml
  rand = "0.8.5"
  ```
* Bring items into scope:

  ```rust
  use rand::Rng;
  ```

## Nested Paths

* Combine paths:

  ```rust
  use std::{cmp::Ordering, io};
  use std::io::{self, Write};
  ```

## Glob Operator

* Bring all public items:

  ```rust
  use std::collections::*;
  ```
* Use sparingly: can obscure where items are defined.

## File Organization

* Declare module with `mod` in parent file.
* Move module code to:

  * `mod foo;` → `src/foo.rs` or `src/foo/mod.rs`
  * `mod bar;` in `foo.rs` → `src/foo/bar.rs` or `src/foo/bar/mod.rs`

## Summary

* Use modules for organization & privacy.
* Use `pub`, `use`, and `mod` to control access and structure.
* Organize large codebases using separate files and crates.
* Follow idiomatic use patterns for clean, maintainable code.
