// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref match_) => println!("Co-ordinates are {},{} ", match_.x, match_.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
