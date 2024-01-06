// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

// I

fn main() {
    let a = [4;400];

    if a.len() >= 100 {
        println!("Wow, that's a big array! {} {:?}",a.len(),a);
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
