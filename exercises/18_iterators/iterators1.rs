// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

#[test]
fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana")); // 参照が指す値を比較する
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: Step 4
}

/*
Rustにおけるイテレータのメモリ状態や動作は、イテレータを作成するメソッドによって異なります。具体的には、`iter()`, `iter_mut()`, `into_iter()` の3つのメソッドで異なる挙動をします。ここではこれらのメソッドがベクターとその要素にどのように作用するか、またメモリ状態はどうなるかについて詳しく解説します。

### `iter()` メソッド

- **参照の生成**:
  `iter()` メソッドは、コレクションの各要素への**不変の参照**(`&T`)を生成します。このイテレータを使用すると、元のベクター(`Vec<T>`)の各要素を指す不変の参照が順番に返されます。

- **メモリ状態**:
  元のベクター(`Vec<String>`)の要素（この場合は`String`）自体はコピーされません。イテレータは元のベクターのメモリに格納されているデータへの参照を保持しています。つまり、追加のメモリを消費することなく、既存のデータを参照するだけです。

### `iter_mut()` メソッド

- **可変参照の生成**:
  `iter_mut()` メソッドは、コレクションの各要素への**可変の参照**(`&mut T`)を生成します。これにより、イテレータを通じてコレクションの各要素を変更することが可能になります。

- **メモリ状態**:
  こちらも元のベクターの要素自体はコピーされず、元のデータへの可変参照が提供されます。データの構造や内容を変更することはできますが、それによって追加のメモリが消費されるわけではありません。

### `into_iter()` メソッド（`Vec<T>`に対する場合）

- **所有権の移動**:
  `into_iter()` メソッドは、コレクション自体の所有権をイテレータに移動します。このイテレータは、元のベクターの各要素の所有権を持ち、それらを値として返します。これにより、イテレータが返す値を直接変更することができます。

- **メモリ状態**:
  元のベクターから要素が移動されるため、元のベクターは空になります（要素は消えますが、容量はそのまま）。イテレータが返す各要素は新しい所有者になりますが、これも新たなメモリ割り当てを伴うわけではありません。ただし、イテレータを通じて返される要素は、以前のベクターから独立して扱うことができます。

これらの説明からわかるように、イテレータは効率的にコレクションを扱うためのツールであり、不必要なデータのコピーを避けることができます。この挙動はメモリ使用を最適化し、パフォーマンスを向上させるのに寄与します。
*/
