# collect_array_ext_trait

[![Crates.io](https://img.shields.io/crates/v/collect_array_ext_trait)](https://crates.io/crates/collect_array_ext_trait)
[![Downloads](https://img.shields.io/crates/d/collect_array_ext_trait.svg)](https://crates.io/crates/collect_array_ext_trait)
[![Documentation](https://docs.rs/collect_array_ext_trait/badge.svg)](https://docs.rs/collect_array_ext_trait)
[![License](https://img.shields.io/crates/l/collect_array_ext_trait)](https://crates.io/crates/collect_array_ext_trait)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/collect_array_ext_trait/status.svg)](https://deps.rs/repo/github/JohnScience/collect_array_ext_trait)

This is a simple library for collecting an instance of `It`: [`std::iter::Iterator<Item=T>`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) into an array `[T; N]` where `N` is the presumed length of the iterator.

## Example

```rust
use collect_array_ext_trait::CollectArray;

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
