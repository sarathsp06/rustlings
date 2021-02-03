// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    ($a:String, $b:bool) => {
        println!("Look at this other macro: {},{}", $a,$b);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
    my_macro!("dd",true);
}
