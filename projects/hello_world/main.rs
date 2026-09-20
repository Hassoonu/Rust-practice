fn main() {
    println!("Hello, World!");
}


// We can create a project using cargo new.
// We can build a project using cargo build.
// We can build and run a project in one step using cargo run.
// We can build a project without producing a binary to check for errors using cargo check.
// Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.


// Even though the hello_cargo project is simple, 
// it now uses much of the real tooling you’ll use in the rest of your Rust career. 
// In fact, to work on any existing projects, 
// you can use the following commands to check out the code using Git, 
// change to that project’s directory, and build:
// $ git clone example.org/someproject
// $ cd someproject
// $ cargo build