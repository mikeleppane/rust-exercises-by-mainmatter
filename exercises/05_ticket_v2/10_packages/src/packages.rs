// This is a `main.rs` file, therefore `cargo` interprets this as the root of a binary target.

// TODO: fix this broken import. Create a new library target in the `src` directory.
//   The library target should expose a public function named `hello_world` that takes no arguments
//   and returns nothing.

// The library target should be named `packages`.
//   The function `hello_world` should print "Hello, world!" to the console.

pub fn hello_world() {
    println!("Hello, world!");
}
