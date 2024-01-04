// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),  // Someの時の世界線においてはmoveされることをpartial moveとしている
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}

// この時の「部分的な」moveの部分とは、直積的な分割ではなく、直和的な分割の上での部分。
// 具体的には、全体`y`を、`Some`と`None`に分割したときに、`Some`の部分だけをmoveする、ということ。Pointの属性であるxとyに分けた意味では無い。
