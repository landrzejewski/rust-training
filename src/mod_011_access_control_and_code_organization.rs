// =================================================================================================
// Section 1: Packages, Crates, and the Module System
// =================================================================================================

/*
## Packages, Crates, and the Module System

Rust organizes code through a three-level hierarchy: **packages**, **crates**, and **modules**.

### Packages

- A package is what `cargo new` creates — a directory containing a `Cargo.toml` file and source
  code.
- `Cargo.toml` manages metadata (name, version, edition), dependencies, build settings, and feature
  flags.
- A package must contain **at least one crate**.
- A package can have **at most one library crate** but any number of binary crates.

### Crates

- A crate is Rust's **compilation unit** — the smallest amount of code the compiler considers at a
  time.
- **Binary crates** compile to an executable. They must have a `main` function. The crate root is
  `src/lib`.
- **Library crates** compile to a reusable library. They have no `main` function. The
  crate root is `src/lib.rs`.
- Additional binary crates can be placed in `src/bin/` — each `.rs` file there becomes a separate
  binary.
- **Binary + library best practice.** When a package has both a binary (`src/lib`) and a library
  (`src/lib.rs`), the idiomatic split is to put the **entire module tree** in the library and keep
  the binary as a **thin wrapper** that just starts the executable and calls into the library's
  public API. This way the binary crate consumes the library the same way any external user would —
  forcing you to exercise your own public API. This very project follows that pattern: `src/lib`
  has an empty `fn main() {}`, while `src/lib.rs` declares `pub mod advanced; pub mod basic; pub mod
  exercises;` — all the training modules live in the library side.
- The `crate` keyword in paths refers to the root of the current crate (i.e., `src/lib` or
  `src/lib.rs`).

### Modules

- Modules are the organizational unit **within** a crate. They group related items (functions,
  structs, enums, constants, traits, other modules) and control visibility.
- Modules form a **tree** rooted at the crate root. Every item lives inside a module — the crate
  root itself is the implicit top-level module.
- Modules can be defined in two ways:
  - **Inline**: `mod name { ... }` with a body in braces, defined directly in the current file.
  - **File-based**: `mod name;` with a semicolon, which tells the compiler to load the module body
    from a separate file.

### File Layout Conventions (for file-based modules)

Two conventions exist for mapping modules to files:

**Method 1 — File per module (preferred)**:
- `src/name.rs` for a module named `name`.
- Submodules go in `src/name/sub.rs`.
- Example: `mod product;` in the crate root loads `src/product.rs`. If `product` has `mod
  category;`, it loads `src/product/category.rs`.

**Method 2 — Folder with mod.rs**:
- `src/name/mod.rs` for a module named `name`.
- Submodules go alongside as `src/name/sub.rs`.
- This is the older convention. It works but leads to many files named `mod.rs`, making navigation
  harder.

**Mixing the two styles for the same module is a compiler error** — you cannot have both
`src/foo.rs` and `src/foo/mod.rs` present for a module named `foo`. Using different styles for
*different* modules in the same project is permitted but discouraged for consistency.

Both methods are valid, but the file method is preferred in modern Rust.

### Module Boundaries

- Module boundaries are defined by the `mod` keyword, **not** by files or folders. A single file can
  contain multiple inline modules, and a single module can span helper files.
- **`mod foo;` is a declaration, not an `#include`.** If you come from C or C++, resist the reflex
  to `mod foo;` from every file that uses `foo`'s items. A module is declared **once** — in its
  parent — and the rest of the crate refers to it by path (`crate::foo::...`, `super::foo::...`, or
  via `use`). Declaring the same module twice (e.g., `mod foo;` in both `lib.rs` and another file)
  creates two unrelated modules with the same name and usually produces confusing
  duplicate-definition errors.
- The rest of this module uses inline modules (`mod name { ... }`) to demonstrate module concepts
  within a single file.

### Visualizing Module Structure

- The `cargo-modules` crate provides a command-line tool for visualizing a crate's module hierarchy
  as a tree.
- Install with `cargo install cargo-modules`, then run `cargo modules structure` to see the module
  tree.
- Useful for navigating, refactoring, and understanding large codebases.
*/

fn packages_crates_and_the_module_system() {
    // A simple inline module — defined right here with a body in braces.
    // Note: modules can be defined inside function bodies for quick demos,
    // but production code defines them at the module (file) level.
    mod greeting {
        pub fn hello() -> &'static str {
            "Hello from an inline module!"
        }

        // This function is private — only accessible within `greeting`
        #[allow(dead_code)]
        fn internal_helper() -> &'static str {
            "I am private to the greeting module"
        }
    }

    // Access the public function using a relative path
    println!("{}", greeting::hello());

    // greeting::internal_helper() would fail — it is private
    // println!("{}", greeting::internal_helper()); // ERROR: function is private

    println!("packages_crates_and_the_module_system section executed");
}

// =================================================================================================
// Section 2: Module Paths and Nesting
// =================================================================================================

/*
## Module Paths and Nesting

Every item in Rust has a **path** — its fully qualified location in the module tree. Paths are how
you refer to functions, types, and other items across modules.

### Path Types

- **Absolute paths** start with `crate::` and trace from the crate root down to the item. Example:
  `crate::basic::mod_011_access_control_and_code_organization::paths_demo::parent_function`.
  Absolute paths are unambiguous but can be very long.
- **Relative paths** start from the current module and use the item name directly. If you are in the
  same module as `paths_demo`, you write `paths_demo::parent_function()`.

### Special Path Keywords

- `self::` explicitly refers to the **current module**. Useful when a local item name might be
  ambiguous or for clarity in macros.
- `super::` refers to the **parent module**. A child module uses `super::` to access items defined
  in its parent. Chaining is valid: `super::super::` goes up two levels.

### Nesting

- Modules can be nested: `mod outer { mod inner { ... } }`. The path to items in `inner` is
  `outer::inner::item`.
- There is no limit to nesting depth, but deep nesting usually signals that the code should be
  reorganized.

### Privacy Boundary

- The **module** is Rust's privacy boundary. Items in a child module can see **everything** in their
  parent (even private items). But a parent cannot see private items in a child — those must be
  marked `pub`.
*/

// Modules defined at file level so that super:: and crate:: resolve
// correctly in the module tree
mod paths_demo {
    // A function in the parent module
    pub fn parent_function() -> &'static str {
        "called from paths_demo (parent)"
    }

    pub mod nested {
        // Access the parent module using super::
        pub fn call_parent() -> String {
            let msg = super::parent_function();
            format!("nested called parent via super: {msg}")
        }

        pub mod deep {
            // super:: goes to nested, super::super:: goes to paths_demo
            pub fn call_grandparent() -> String {
                let msg = super::super::parent_function();
                format!("deep called grandparent via super::super: {msg}")
            }
        }
    }

    // self:: explicitly refers to the current module
    pub fn use_self() -> String {
        format!("self:: demo: {}", self::parent_function())
    }
}

fn module_paths_and_nesting() {
    // Relative path — paths_demo is a sibling at file-module level
    println!("{}", paths_demo::parent_function());

    // Nested module access
    println!("{}", paths_demo::nested::call_parent());

    // Deeply nested — demonstrates super::super::
    println!("{}", paths_demo::nested::deep::call_grandparent());

    // self:: demonstration
    println!("{}", paths_demo::use_self());

    // Absolute paths start with crate:: and trace from the crate root.
    // The full path to parent_function would be:
    //   crate::basic::mod_011_access_control_and_code_organization
    //       ::paths_demo::parent_function()
    // This is valid but extremely verbose — relative paths are preferred
    // when accessing items nearby in the module tree.

    println!("module_paths_and_nesting section executed");
}

// =================================================================================================
// Section 3: Visibility Modifiers
// =================================================================================================

/*
## Visibility Modifiers

By default, all items in Rust are **private** — visible only within the module where they are
defined and its descendant submodules. Visibility modifiers control who else can access an item.

### Modifier Summary

- **(default)** — private. Accessible only within the defining module and its children.
- **`pub`** — fully public. Accessible to any code that can reach the parent module. In a library
  crate, `pub` items are part of the public API.
- **`pub(crate)`** — accessible anywhere within the **same crate** but not to external crates. The
  most common restricted visibility in library code — useful for internal sharing.
- **`pub(super)`** — accessible only to the **parent module** (and that parent's descendants).
  Useful for sharing with the immediate parent without exposing to the entire crate.
- **`pub(self)`** — equivalent to private (the default). Exists for completeness and is occasionally
  useful in macro-generated code.
- **`pub(in path)`** — accessible within a specific ancestor module. Example: `pub(in
  crate::some_module)`. Useful when a deeply nested module wants to expose helpers to a specific
  ancestor without making them `pub(crate)`. Rarely needed in practice.

### Key Rules

- **Making a module `pub` does not make its contents public.** Each function, struct, or other item
  inside must independently be marked `pub` to be accessible from outside.
- **Child modules can access all items in their parent**, even private ones. Privacy only restricts
  access from *outside* the module boundary — a parent looking into a child, or one sibling looking
  at another.
- **Siblings** (modules at the same level) cannot see each other's private items — they can only
  access each other's `pub` items via the parent's namespace (e.g.,
  `super::sibling_name::pub_item()`).
*/

mod visibility_demo {
    // Private by default — only visible within visibility_demo and its children
    fn private_fn() -> &'static str {
        "I am private"
    }

    // pub — visible to anyone who can access visibility_demo
    pub fn public_fn() -> &'static str {
        "I am public"
    }

    // pub(super) — visible to the parent of visibility_demo (the file-level module)
    pub(super) fn super_visible_fn() -> &'static str {
        "I am pub(super)"
    }

    // pub(crate) — visible anywhere in this crate
    pub(crate) fn crate_visible_fn() -> &'static str {
        "I am pub(crate)"
    }

    // pub(self) — equivalent to private, shown for completeness
    #[allow(dead_code)]
    pub(self) fn self_visible_fn() -> &'static str {
        "I am pub(self), same as private"
    }

    pub mod child {
        // A child module CAN access its parent's private items
        pub fn access_parent_private() -> String {
            // super:: reaches into visibility_demo — child sees everything
            format!("child sees parent's private fn: {}", super::private_fn())
        }
    }

    pub mod sibling_a {
        pub fn greet() -> &'static str {
            "hello from sibling_a"
        }

        // Private to sibling_a — sibling_b cannot access this
        #[allow(dead_code)]
        fn secret() -> &'static str {
            "sibling_a's secret"
        }
    }

    pub mod sibling_b {
        // Siblings access each other's PUBLIC items through the parent
        pub fn call_sibling() -> String {
            format!("sibling_b calls: {}", super::sibling_a::greet())
        }

        // Cannot access sibling_a::secret() — it is private to sibling_a
        // super::sibling_a::secret() // ERROR: function is private
    }
}

fn visibility_modifiers() {
    // pub — accessible
    println!("{}", visibility_demo::public_fn());

    // pub(super) — accessible because this function is in the parent module
    println!("{}", visibility_demo::super_visible_fn());

    // pub(crate) — accessible because we are in the same crate
    println!("{}", visibility_demo::crate_visible_fn());

    // Default private — NOT accessible from here:
    // println!("{}", visibility_demo::private_fn()); // ERROR: function is private

    // pub(self) — also NOT accessible from here (equivalent to private):
    // println!("{}", visibility_demo::self_visible_fn()); // ERROR: function is private

    // Child accessing parent's private function — this works
    println!("{}", visibility_demo::child::access_parent_private());

    // Sibling accessing sibling's public function through parent
    println!("{}", visibility_demo::sibling_b::call_sibling());

    println!("visibility_modifiers section executed");
}

// =================================================================================================
// Section 4: Struct and Enum Privacy
// =================================================================================================

/*
## Struct and Enum Privacy

Visibility rules apply not just to functions and modules but also to the **fields** of structs and
the **variants** of enums. Structs and enums follow different rules.

### Struct Field Privacy

- Making a struct `pub` makes the **type name** visible, but its **fields remain private** by
  default.
- Each field must be individually marked `pub` to be accessible from outside the module.
- A struct with any private fields **cannot be constructed** with struct literal syntax from outside
  its module — the constructor would need access to the private fields.
- The common pattern is to provide a **constructor function** (typically `fn new(...)`) in an `impl`
  block within the same module. The constructor has access to private fields because it is inside
  the module.
- **Getter methods** provide read-only access to private fields when needed.
- **Invariant maintenance through methods**: a common use of private fields is to keep multiple
  fields in sync. For example, a struct could store a `Vec<i32>` together with a cached `f64`
  average of its values. Both fields are private, and every mutating method (`add`, `remove`) calls
  a private helper (`update_average`) that keeps the cache consistent. External code sees only the
  public `add`/`remove`/`average` API and cannot desynchronize the two fields by poking at them
  directly. This also means the internal representation can change later (e.g., switching from
  `Vec<i32>` to `BTreeSet<i32>`) without breaking any callers — the public method signatures stay
  the same.

### Enum Variant Privacy

- Making an enum `pub` makes the type **and all its variants** public. Enum variants do not have
  individual visibility settings.
- This is a deliberate design choice: `match` requires **exhaustive** pattern matching over all
  variants. If some variants were hidden, code outside the module could not write complete `match`
  expressions.
- This is the key asymmetry: struct fields can be individually public or private; enum variants are
  always all-or-nothing.
*/

mod privacy_demo {
    // Struct with mixed field visibility
    pub struct User {
        pub username: String, // public — accessible from outside
        email: String,        // private — only accessible within privacy_demo
        login_count: u32,     // private
    }

    impl User {
        // Constructor — the only way to create a User from outside this module,
        // since email and login_count are private fields
        pub fn new(username: String, email: String) -> Self {
            Self {
                username,
                email,
                login_count: 0,
            }
        }

        // Getter — provides read-only access to a private field
        pub fn email(&self) -> &str {
            &self.email
        }

        // Method that modifies private state
        pub fn record_login(&mut self) {
            self.login_count += 1;
        }

        pub fn login_count(&self) -> u32 {
            self.login_count
        }
    }

    // Enum — all variants become public when the enum is pub
    #[allow(dead_code)]
    pub enum Permission {
        Read,
        Write,
        Admin(String), // variant data is also accessible
    }
}

fn struct_and_enum_privacy() {
    // Cannot construct User with struct literal from outside the module
    // because email and login_count are private:
    // let u = privacy_demo::User {
    //     username: String::from("alice"),
    //     email: String::from("alice@example.com"), // ERROR: field is private
    //     login_count: 0,                           // ERROR: field is private
    // };

    // Must use the constructor
    let mut user =
        privacy_demo::User::new(String::from("alice"), String::from("alice@example.com"));

    // Public field — direct access works
    println!("username: {}", user.username);

    // Private field — must use getter
    println!("email: {}", user.email());
    // user.email — ERROR: field `email` is private

    // Modify private state through a public method
    user.record_login();
    user.record_login();
    println!("login count: {}", user.login_count());

    // Enum variants are all public — pattern matching works from outside
    let perm = privacy_demo::Permission::Admin(String::from("root"));
    match perm {
        privacy_demo::Permission::Read => println!("read-only access"),
        privacy_demo::Permission::Write => println!("read-write access"),
        privacy_demo::Permission::Admin(name) => println!("admin access: {name}"),
    }

    // Note: the #[non_exhaustive] attribute, when placed on a pub enum or
    // pub struct, forces external crates to include a wildcard arm (`_ =>`)
    // in any `match` expression and prevents them from constructing the
    // type with struct-literal syntax. This lets library authors add new
    // variants or fields later without breaking downstream code. Within
    // the same crate, the attribute has no effect.

    println!("struct_and_enum_privacy section executed");
}

// =================================================================================================
// Section 5: The `use` Declaration and Re-exporting
// =================================================================================================

/*
## The `use` Declaration and Re-exporting

### The `use` Declaration

- `use path::to::Item;` brings an item into the current scope, avoiding repeated long paths. It is
  conventionally described as "bringing into scope" rather than "importing" — the item is not
  copied, just the name becomes available locally.
- `use` can appear at the top of a file (module level) or inside a function body. Placing it at the
  module level is more common.
- **`use` is scope-local.** A `use` statement only creates the shortcut for the module (or function
  body) in which it appears — it does **not** propagate into child modules. If you have `use
  crate::foo::bar;` at the crate root and then define `mod child { ... }`, code inside `child` does
  not see `bar` — `child` must add its own `use` or reference the item by path (`super::bar` or
  `crate::foo::bar`). This can surprise newcomers who expect imports to behave like file-level
  globals.
- **Nested use**: `use path::to::{ItemA, ItemB};` brings multiple items from the same path in one
  statement.
- **Self in nested use**: `use path::to::{self, Item};` brings both the module itself and an item
  from it.
- **Renaming**: `use path::to::Item as Alias;` renames the item locally with the `as` keyword.
  Useful for resolving name conflicts or improving clarity.
- **Glob import**: `use path::to::*;` brings everything public from that path into scope. Generally
  discouraged in production code because it obscures where names come from. Acceptable in test
  modules (`use super::*;`) and preludes.
- **The Prelude**: Rust automatically imports commonly used items into every module via
  `std::prelude`. This is why `Option`, `Result`, `String`, `Vec`, `Some`, `None`, `Ok`, `Err`, and
  traits like `Drop`, `Clone`, `Copy`, `Iterator` work without `use`. Each Rust edition may add
  items to the prelude (e.g., the 2021 edition added `TryFrom`/`TryInto`).

### Idiomatic `use` Conventions

- For **functions**, bring the parent module into scope: `use std::io;` then `io::stdin()`. This
  makes it clear where the function comes from.
- For **structs, enums, and traits**, bring the type directly: `use std::collections::HashMap;` then
  `HashMap::new()`. Type names are usually unambiguous.
- When two types have the **same name**, either use the parent module for one or both, or rename
  with `as`.

### Re-exporting (`pub use`)

- `pub use path::to::Item;` makes an item available under the current module's path, as if it were
  defined here. Consumers see a clean public API without knowing the internal structure.
- `pub use` can be combined with `as` for renaming: `pub use inner::LongName as ShortName;`.
- Re-exporting **does not break privacy** — you can only `pub use` items that are already publicly
  accessible from the re-exporting module's perspective.
- A common pattern in library crates: organize code into many small internal modules, then `pub use`
  the key types from the crate root to give users a flat, convenient API.
*/

mod use_demo {
    pub mod shapes {
        pub struct Circle {
            pub radius: f64,
        }

        impl Circle {
            pub fn area(&self) -> f64 {
                std::f64::consts::PI * self.radius * self.radius
            }
        }

        pub struct Rectangle {
            pub width: f64,
            pub height: f64,
        }

        impl Rectangle {
            pub fn area(&self) -> f64 {
                self.width * self.height
            }
        }
    }

    pub mod colors {
        pub struct Rgb(pub u8, pub u8, pub u8);

        impl Rgb {
            pub fn as_string(&self) -> String {
                format!("rgb({}, {}, {})", self.0, self.1, self.2)
            }
        }
    }

    // Re-export: expose shapes::Circle directly from use_demo
    // Consumers write use_demo::Circle instead of use_demo::shapes::Circle
    pub use shapes::Circle;

    // Re-export with rename
    pub use colors::Rgb as Color;
}

fn use_declaration_and_reexporting() {
    // Without use — full path every time (verbose but unambiguous)
    let c1 = use_demo::shapes::Circle { radius: 5.0 };
    println!("circle area (full path): {:.2}", c1.area());

    // With use — bring a single item into scope
    use use_demo::shapes::Rectangle;
    let rect = Rectangle {
        width: 4.0,
        height: 6.0,
    };
    println!("rectangle area (use): {:.2}", rect.area());

    // Nested use — bring multiple items from the same path
    use use_demo::shapes::{Circle, Rectangle as Rect};
    let c2 = Circle { radius: 3.0 };
    let r2 = Rect {
        width: 2.0,
        height: 8.0,
    };
    println!("circle: {:.2}, rect: {:.2}", c2.area(), r2.area());

    // as keyword for local renaming
    use use_demo::colors::Rgb as RgbColor;
    let blue = RgbColor(0, 0, 255);
    println!("aliased color: {}", blue.as_string());

    // Re-exported item — available directly from use_demo
    // (no need to know about the internal shapes submodule)
    let c3 = use_demo::Circle { radius: 1.0 };
    println!("re-exported circle area: {:.2}", c3.area());

    // Re-exported with rename — Color is Rgb under a different name
    let red = use_demo::Color(255, 0, 0);
    println!("re-exported color: {}", red.as_string());

    println!("use_declaration_and_reexporting section executed");
}

// =================================================================================================
// Section 6: Documentation Comments
// =================================================================================================

/*
## Documentation Comments

Rust has first-class support for documentation through special comment syntax that integrates with
`cargo doc`.

### Outer Doc Comments (`///`)

- `///` creates a documentation comment for the **next item** — a function, struct, enum, trait,
  module, constant, etc.
- Each `///` line is a separate doc comment line. Together they form the item's documentation.
- Doc comments support full **Markdown**: headings, bold, italic, code spans, links, lists, and
  fenced code blocks.

### Inner Doc Comments (`//!`)

- `//!` creates a documentation comment for the **enclosing item** — typically used at the top of
  `lib.rs` to document the crate, or at the top of a module file to document the module.
- Think of `///` as "document what follows" and `//!` as "document what contains me".

### Doc Tests

- Fenced code blocks (` ``` `) in doc comments are compiled and run as tests by `cargo test`. This
  ensures examples stay correct as the code evolves.
- Lines starting with `# ` inside doc code blocks are **hidden** from rendered documentation but
  still compiled. Used to include setup code (like `use` statements) without cluttering the example.
- Use ` ```no_run ``` ` for code that should compile but not execute (e.g., code that requires
  network access).
- Use ` ```ignore ``` ` for code that should not be compiled at all (e.g., pseudocode).
- Use ` ```text ``` ` for non-Rust content shown as plain text.

### Generating Documentation

- `cargo doc` generates HTML documentation for the crate and its dependencies. `cargo doc --open`
  opens it in a browser.
- Only `pub` items appear in the generated documentation. Private items are excluded by default (use
  `--document-private-items` to include them).

### Conventional Sections

Doc comments commonly use these markdown headings (by convention):
- `# Examples` — usage examples (the most important section).
- `# Panics` — conditions under which the function panics.
- `# Errors` — for functions returning `Result`, describes error conditions.
- `# Safety` — for `unsafe` functions, explains the invariants the caller must uphold.

### Attribute Form

- Doc comments are syntactic sugar for the `#[doc = "..."]` attribute. `/// Hello` is equivalent to
  `#[doc = " Hello"]`. The attribute form is rarely written by hand but is useful in macros.
- **Intra-doc links**: use `[`Type`]` or `[`method`](Type::method)` in doc comments to create
  hyperlinks to other items. The compiler verifies these links exist, catching broken references.

### Additional Documentation Attributes

- **`#[doc(alias = "name")]`** adds a search alias so a user can find the item by typing `name` in
  the rustdoc search box even when `name` does not appear in the item's identifier. Example: a
  `send_request` helper can use `#[doc(alias = "fetch")]` to surface for users who learned the word
  "fetch" in another ecosystem. The alias does not affect code — it only shows up in the generated
  documentation.
- **`#![doc = include_str!("../README.md")]`** at the top of `lib.rs` pulls the project's README
  into the crate-level documentation. This keeps the crate's README and the rustdoc front page in
  sync without duplicating content. The trick combines the inner doc-attribute form (`#![doc =
  "..."]`) with the compile-time `include_str!` macro covered in module 005 section 5c.
*/

/// Fetches data over the network (stub used to demonstrate the attributes below).
///
/// # Examples
///
/// ```ignore
/// let _ = send_request();
/// ```
#[allow(dead_code)]
#[doc(alias = "fetch")]
#[doc(alias = "http_get")]
fn send_request() -> &'static str {
    "200 OK"
}

/// A temperature measurement with its unit.
///
/// `Temperature` stores a value as `f64` and tracks whether it is
/// in Celsius or Fahrenheit. Conversion methods are provided.
///
/// # Examples
///
/// ```ignore
/// // In a library crate, this would be:
/// // use my_crate::doc_demo::Temperature;
/// let boiling = Temperature::Fahrenheit(212.0);
/// assert!((boiling.to_celsius() - 100.0).abs() < f64::EPSILON);
/// ```
///
/// Note: doc tests compile and run via `cargo test` for library crates.
/// These examples use `ignore` to keep the code readable without the
/// verbose `use crate::basic::mod_011_access_control_and_code_organization::doc_demo::Temperature;`
/// import that would otherwise be required to bring `Temperature` into scope.
/// Lines prefixed with `# ` are compiled but hidden in rendered docs.
mod doc_demo {
    //! This module demonstrates documentation comment syntax.
    //!
    //! Inner doc comments (`//!`) like these describe the enclosing
    //! module. They appear at the top of the module's documentation
    //! page when generated with `cargo doc`.

    /// Represents a temperature measurement.
    ///
    /// Each variant stores the temperature value as `f64`.
    pub enum Temperature {
        /// Temperature in degrees Celsius.
        Celsius(f64),
        /// Temperature in degrees Fahrenheit.
        Fahrenheit(f64),
    }

    impl Temperature {
        /// Converts the temperature to Celsius.
        ///
        /// If already in Celsius, the value is returned unchanged.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// let t = Temperature::Fahrenheit(212.0);
        /// assert!((t.to_celsius() - 100.0).abs() < f64::EPSILON);
        /// ```
        pub fn to_celsius(&self) -> f64 {
            match self {
                Temperature::Celsius(c) => *c,
                Temperature::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0,
            }
        }

        /// Converts the temperature to Fahrenheit.
        ///
        /// If already in Fahrenheit, the value is returned unchanged.
        pub fn to_fahrenheit(&self) -> f64 {
            match self {
                Temperature::Celsius(c) => c * 9.0 / 5.0 + 32.0,
                Temperature::Fahrenheit(f) => *f,
            }
        }

        /// Returns a human-readable string representation.
        pub fn as_string(&self) -> String {
            match self {
                Temperature::Celsius(c) => format!("{c:.1}°C"),
                Temperature::Fahrenheit(f) => format!("{f:.1}°F"),
            }
        }
    }
}

fn documentation_comments() {
    use doc_demo::Temperature;

    let boiling = Temperature::Fahrenheit(212.0);
    println!("boiling point: {}", boiling.as_string());
    println!("  in Celsius: {:.1}°C", boiling.to_celsius());

    let freezing = Temperature::Celsius(0.0);
    println!("freezing point: {}", freezing.as_string());
    println!("  in Fahrenheit: {:.1}°F", freezing.to_fahrenheit());

    let body_temp = Temperature::Celsius(37.0);
    println!("body temperature: {}", body_temp.as_string());
    println!("  in Fahrenheit: {:.1}°F", body_temp.to_fahrenheit());

    // Generate docs with: cargo doc --open
    // Run doc tests with: cargo test
    // Only pub items appear in the generated documentation.

    println!("documentation_comments section executed");
}

// =================================================================================================
// Section 7: External Dependencies and Publishing
// =================================================================================================

/*
## External Dependencies and Publishing

This section covers topics that cannot be demonstrated in a single file but are essential to Rust's
code organization story.

### External Dependencies

- **crates.io** is Rust's public package registry where developers publish and discover crates. Each
  crate page provides documentation, version history, and popularity indicators.
- Dependencies are added under `[dependencies]` in `Cargo.toml`:
  ```toml
  [dependencies]
  serde = "1.0"
  tokio = { version = "1", features = ["full"] }
  ```
- Running `cargo build` automatically fetches and compiles dependencies. Items from a dependency are
  brought into scope with the `use` keyword, e.g., `use serde::Serialize;`.
- Dependencies improve reusability by providing well-tested solutions for common tasks. Using
  community-standard crates also enhances code readability and consistency.

### Choosing Dependencies

- Prefer crates that are **actively maintained** and **well documented**.
- Avoid excessive dependencies — each crate increases compile time and binary size.
- Prefer crates that are easy to understand, especially when internal behavior matters for
  correctness or security.

### Preparing for Publishing

- The crate root (`lib.rs`) can be documented using `//!` comments at the top of the file. These
  describe the crate as a whole and appear on the crate's documentation front page.
- `Cargo.toml` should include metadata fields to avoid warnings and provide information to users:
  - `description` — a brief summary of what the crate does.
  - `license` — the SPDX license identifier (e.g., `"MIT"`, `"Apache-2.0"`).
  - `homepage` — URL of the project's homepage.
  - `repository` — URL of the source code repository.
- The package name must be **unique** on crates.io — no other published crate can share the same
  name.

### Publishing and Versioning

- `cargo publish` uploads the package to crates.io. It then appears under the user's dashboard.
- Publishing requires logging in with a GitHub account and generating an API token on crates.io. The
  token is saved with `cargo login <token>` into `~/.cargo/credentials.toml` (local to the machine,
  and readable only by the owner). Tokens should be kept secret or revoked on crates.io if leaked.
- All changes in the working directory must be committed, or publish with `--allow-dirty` (not
  recommended).
- Published versions **cannot be deleted**, but a version can be **yanked** (`cargo yank --vers
  1.0.0`) to prevent new projects from depending on it. Existing projects that already depend on a
  yanked version are unaffected. A yank can be reversed with `cargo yank --vers 1.0.0 --undo`,
  allowing new projects to depend on that version again. Yanking never removes code — it only blocks
  new resolutions.
- New releases require updating the `version` field in `Cargo.toml` before publishing again. Rust
  crates follow **semantic versioning** (see semver.org): the `MAJOR.MINOR.PATCH` bump is chosen
  based on the kind of change — breaking changes bump MAJOR, backwards-compatible additions bump
  MINOR, and backwards-compatible bug fixes bump PATCH.

### Workspaces

- **Workspaces** group multiple related packages under a single root `Cargo.toml`. Members share one
  `Cargo.lock` and `target/` directory, ensuring consistent dependency versions.
- The root `Cargo.toml` contains a `[workspace]` table listing the members and — for current Rust —
  the recommended resolver:
  ```toml
  [workspace]
  resolver = "3"
  members = ["crate-a", "crate-b"]
  ```
  Each member has its own `Cargo.toml`.
- Cargo does **not** automatically assume dependencies between workspace members — a member that
  wants to use another member must declare it as a **path dependency** in its own `Cargo.toml`:
  ```toml
  [dependencies]
  add_one = { path = "../add_one" }
  ```
- The shared `Cargo.lock` guarantees that every member uses the same version of each external
  dependency (for example `rand`), but each member must still **independently declare** that
  dependency in its own `Cargo.toml` before it can `use` it — appearing in another member's file is
  not enough.
- Build a specific member with `cargo build -p crate-a`, or the whole workspace with `cargo build`
  from the root. Similarly, `cargo test` runs the tests of all members, while `cargo test -p
  crate-a` runs tests only for one member.

### Feature Flags

- **Feature flags** enable conditional compilation of optional functionality. Defined in
  `Cargo.toml` under `[features]`. Dependencies can be feature-gated with `optional = true`. Enable
  features from consumers: `my_crate = { version = "1.0", features = ["json"] }` Use `#[cfg(feature
  = "json")]` in code to conditionally compile. The `default` feature lists what is enabled by
  default.

### Release Profiles

- **Release profiles** control compiler optimization and debug settings. Cargo has two built-in
  profiles: `dev` (used by `cargo build`) and `release` (used by `cargo build --release`).
- Defaults: `dev` uses `opt-level = 0` (fast compilation, no optimization) and includes debug info.
  `release` uses `opt-level = 3` (maximum optimization) and strips debug info.
- Override defaults in `Cargo.toml`: `[profile.dev]` `opt-level = 1` `[profile.release]` `opt-level
  = 2` `lto = true`
- Key settings: `opt-level` (0–3, or "s"/"z" for size), `lto` (link- time optimization — slower
  builds, faster binaries), `codegen-units` (lower = better optimization, slower build), `strip`
  (remove symbols from binary), `debug` (debug info level), `overflow-checks` (on by default in dev,
  off in release).
- Custom profiles can inherit from built-in ones: `[profile.bench]` `inherits = "release"` `debug =
  true`

### Installing Binaries and Custom Subcommands

- `cargo install <crate>` compiles a crate from crates.io and installs its executable into
  `$HOME/.cargo/bin`. That directory must be on `$PATH` for the installed binary to be callable from
  any shell. Only crates with a **binary target** — a `src/lib` or an explicit `[[bin]]` entry
  in `Cargo.toml` — can be installed this way; library-only crates cannot. Example: `cargo install
  ripgrep` installs the `rg` executable. This repository uses the same mechanism to install
  developer tooling such as `cargo-modules` and `cargo-expand`.
- **Custom Cargo subcommands**: any executable in `$PATH` whose filename starts with `cargo-` (e.g.
  `cargo-expand`) can be invoked as though it were a built-in Cargo subcommand — `cargo expand` in
  this case. `cargo --list` shows every subcommand available in the current environment, including
  both built-in and custom ones. The design makes Cargo extensible without having to modify Cargo
  itself. Common installable subcommands include `cargo-expand` (expand macro invocations),
  `cargo-watch` (re-run a command on source changes), and `cargo-edit` (modify `Cargo.toml` from the
  CLI).

### Production Workflow Knobs

The following items round out the Cargo workflow story from the rest of this section. They are all
configuration-level topics (no runnable Rust code), but knowing they exist changes how you set up a
real project.

- **MSRV — Minimum Supported Rust Version**. Cargo supports a `rust-version` field in `[package]` to
  declare the oldest toolchain the crate compiles with:
  ```toml
  [package]
  name = "my_crate"
  version = "0.1.0"
  edition = "2024"
  rust-version = "1.75.0"
  ```
  Cargo will warn (not error) when the active `rustc` is older than the declared MSRV. Libraries use
  this to signal compatibility; downstream users respect it when choosing a toolchain.

- **`rust-toolchain.toml`** pins the toolchain at the *Rustup* level, which is stricter than MSRV.
  When present in the project root, running any `cargo` command in that directory automatically uses
  the specified channel (plus any requested components and targets):
  ```toml
  [toolchain]
  channel = "1.82.0"
  components = ["rustfmt", "clippy"]
  targets = ["wasm32-unknown-unknown"]
  ```
  Rustup installs the requested toolchain on first use. Use this when a project depends on a
  specific stable release, on nightly for a particular feature, or on non-default components.

- **Build profile customization**. Beyond the defaults mentioned earlier in this section, a common
  release-profile tweak is to keep some debug information in the final binary — you want backtraces
  and flame graphs without giving up optimization:
  ```toml
  [profile.release]
  debug = "limited"
  ```
  Other common tunings: `lto = "thin"` (thin LTO — a cheaper form of link-time optimization), `strip
  = "symbols"` (shrink the binary by dropping symbol tables), and `codegen-units = 1` (slowest to
  compile, best-optimized output for benchmarks).

- **`build.rs` build scripts**. Placing a file named `build.rs` at the package root tells Cargo to
  compile and run it *before* compiling the crate itself. The script prints commands on its stdout
  (`println!("cargo::rustc-env=KEY=VALUE");` and similar) to influence the main build. A typical use
  is baking the current Git commit hash into the binary at compile time:
  ```rust
  // build.rs
  fn main() {
      let output = std::process::Command::new("git")
          .args(["rev-parse", "HEAD"])
          .output();
      if let Ok(out) = output {
          let hash = String::from_utf8_lossy(&out.stdout);
          println!("cargo::rustc-env=COMMIT_HASH={}", hash.trim());
      }
  }
  ```
  The main crate can then read the value with `env!("COMMIT_HASH")` (see module 005 section 5c for
  the `env!` / `option_env!` / `include_str!` family).

- **Git and path dependencies**. Beyond crates.io, a dependency entry can point at a Git repository
  or a local path:
  ```toml
  [dependencies]
  some_lib = { git = "https://github.com/user/some_lib.git",
               tag = "v1.2.3" }
  # or: rev = "a1b2c3d", or: branch = "main"

  internal_util = { path = "../internal_util" }
  ```
  Git dependencies are useful for crates that aren't published yet or for pinning an upstream fix.
  Path dependencies are the workspace-internal analogue — one workspace member depending on another.
  Always pin Git deps with `rev` or `tag` for reproducibility; a bare `branch = "main"` can silently
  change between builds.

- **`Cargo.lock` semantics**. `Cargo.lock` records the exact version of every transitive dependency
  that was resolved on the last successful build. The guidance splits by crate kind:
  - **Binary crates** (and workspaces that produce a binary) should **commit** `Cargo.lock`.
    Everyone building the project resolves to the same set of crates, so CI, local dev, and release
    builds are bit-for-bit equivalent from Cargo's point of view.
  - **Library crates** traditionally **did not** commit `Cargo.lock`, on the theory that downstream
    consumers pick their own versions through *their* lock file. Modern guidance has softened:
    committing `Cargo.lock` for a library fixes CI builds to a known-good set of dependencies, which
    is often worth the trade-off. What matters is that the library's downstream consumers always
    prefer their own `Cargo.lock` — the library's lock file only affects *its own* test runs.
*/

// =================================================================================================
// Public entry point
// =================================================================================================

pub fn run() {
    packages_crates_and_the_module_system();
    module_paths_and_nesting();
    visibility_modifiers();
    struct_and_enum_privacy();
    use_declaration_and_reexporting();
    documentation_comments();
}
