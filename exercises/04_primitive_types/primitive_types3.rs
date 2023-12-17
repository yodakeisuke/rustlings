// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let a: Vec<i32> = (0..1000).collect(); // 範囲リテラルでイテレータを生成.Vec<i32>コレクション型に変換
    // .collect()したものは型アノテーション必須。なぜなら、a: : HashSet<i32> のようにも出来るから。（推論ではなくプログラマが決めること）

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
