# Vecdeque

A way to sort a `VecDeque` in rust.

This repository is a different implementation of https://github.com/rust-lang/rust/pull/69400.

While the other version first shifts the elements of the deque so they are one continuous slice which can be sorted
using `slice::sort`. This implementation sorts both slices individually and then does one recursive merge at the end.

# License

This repository is licensed under [UNLICENSE](UNLICENSE)
