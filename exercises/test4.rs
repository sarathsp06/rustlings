// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

#[macro_use]
macro_rules! my_macro {
    ($val:expr) => {
        match $val {
            "world" => "Hello world!",
            _ => "Good bye, World",
        }

    };
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
