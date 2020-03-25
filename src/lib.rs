#![feature(
    exact_size_is_empty,
    const_generics,
    dropck_eyepatch,
    try_trait,
    raw_vec_internals,
    try_reserve
)]
#![allow(incomplete_features)]
#![allow(soft_unstable)]

extern crate alloc;

mod vec_deque;
