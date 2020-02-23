#![no_main]
use libfuzzer_sys::fuzz_target;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct S(u8, u64);

fuzz_target!(|data: &[u8]| {
    if let Some((&first, rest)) = data.split_first() {
        let mut pos = 0;
        let vec: Vec<S> = rest.chunks_exact(2).map(|c| {
            let item = S(c[0], pos);
            pos += 1;
            item
        }).collect();

        let mut q = VecDeque::with_capacity(rest.len());
        for i in (0..first as usize).rev() {
            if let Some(&item) = vec.get(i) {
                q.push_front(item);
            }
        }

        for &e in vec.iter().skip(first as usize) {
            q.push_back(e);
        }

        let s = format!("{:?}", q.as_slices());
        let mut vec = vec;
        vec.sort_by(|a, b| a.0.cmp(&b.0));

        vecdeque::sort_by(&mut q, |a, b| a.0.cmp(&b.0));
        let q_vec: Vec<S> = Vec::from(q);
        assert_eq!(q_vec, vec, "{}", s);
    }
});
