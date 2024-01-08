// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM  DONE

fn main() { 
    let data = "Rust is great!".to_string();

    let c = get_char(&data);
    println!("cc = {}", c);
   let data2 =  string_uppercase(&data);
    println!("data = {}", data);
    println!("data = {:?}",data2);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase( data: &String) -> String{
    let data = data.to_uppercase();

    println!("{}", data);
    return data;
}
