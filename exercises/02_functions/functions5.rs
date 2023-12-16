// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
    // または return num * num;
}

// NOTE: Rustのセミコロンには明確な意味があり、「式を文に変換する。つまりその式の値を返したりしない。」