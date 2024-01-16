# collect_array

[![Crates.io](https://img.shields.io/crates/v/collect_array)](https://crates.io/crates/collect_array)
[![Downloads](https://img.shields.io/crates/d/collect_array.svg)](https://crates.io/crates/collect_array)
[![Documentation](https://docs.rs/collect_array/badge.svg)](https://docs.rs/collect_array)
[![License](https://img.shields.io/crates/l/collect_array)](https://crates.io/crates/collect_array)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/collect_array/status.svg)](https://deps.rs/repo/github/JohnScience/collect_array)

This is a simple library for collecting an instance of `It`: [`std::iter::Iterator<Item=T>`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) into an array `[T; N]` where `N` is the presumed length of the iterator.

## Example

```rust
use collect_array::CollectArray;

fn main() {
    let arr = (0..5).collect_array::<5>().unwrap();
    let mut iter = arr.iter().copied();
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), None);
}
```
