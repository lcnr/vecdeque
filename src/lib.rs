use std::cmp::{Ord, Ordering};
use std::collections::VecDeque;
use std::mem;

pub fn sort<T: Ord>(q: &mut VecDeque<T>) {
    sort_by(q, |a, b| a.cmp(b))
}

fn merge<'a, T, F>(mut front: &'a mut [T], mut back: &'a mut [T], compare: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    if back.is_empty() {
        return;
    }

    while let Some(initial_element) = front
        .iter()
        .position(|e| compare(&back[0], e) == Ordering::Less)
    {
        mem::swap(&mut back[0], &mut front[initial_element]);

        let mut back_pos = 1;
        for front_pos in (initial_element + 1)..front.len() {
            if let Some(back_elem) = back.get(back_pos) {
                if compare(&back_elem, &back[0]) == Ordering::Less {
                    mem::swap(&mut back[back_pos], &mut front[front_pos]);
                    back_pos += 1;
                } else {
                    mem::swap(&mut back[0], &mut front[front_pos]);
                }
            } else {
                // shift `buffer` to `front[front_pos]` by backshifting
                // all remaining elements in `front`.
                front = &mut front[front_pos..];
                while back.len() <= front.len() {
                    back.swap_with_slice(&mut front[..back.len()]);
                    front = &mut front[back.len()..];
                }

                back[..front.len()].swap_with_slice(front);
                for i in 0..(back.len() - front.len()) {
                    back.swap(i, i + front.len());
                }

                return;
            }
        }

        let (buffer, rest) = back.split_at_mut(back_pos);
        if rest.is_empty() {
            return;
        }

        front = buffer;
        back = rest;
    }
}

pub fn sort_by<T, F>(q: &mut VecDeque<T>, mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let (front, back) = q.as_mut_slices();
    front.sort_by(&mut compare);
    back.sort_by(&mut compare);

    merge(front, back, &mut compare);
}

#[cfg(test)]
mod tests {
    macro_rules! q {
        ($($p:expr),*; $($a:expr),*) => {
            {
                let mut q = VecDeque::with_capacity(63);
                let mut v = vec![$($p),*];
                while let Some(e) = v.pop() {
                    q.push_front(e);
                }
                $(
                    q.push_back($a);
                )*
                q
            }
        };
    }

    fn eq<T: Eq + std::fmt::Debug>(q: VecDeque<T>, t: VecDeque<T>) {
        assert_eq!(q.as_slices(), t.as_slices());
    }

    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct S(u32, u32);

    #[test]
    fn simple() {
        let mut q = q![2, 0; 1];
        sort(&mut q);
        eq(q, q![0, 1; 2])
    }

    #[test]
    fn fuzz1() {
        let mut q = q![1, 2; 0];
        sort(&mut q);
        eq(q, q![0, 1; 2]);
    }

    #[test]
    fn stable() {
        let mut q = q![S(0, 0), S(1, 0), S(0, 1), S(1, 1); S(0, 2), S(0, 3), S(1, 2), S(0, 4)];
        sort_by(&mut q, |a, b| a.0.cmp(&b.0));
        eq(
            q,
            q![S(0, 0), S(0, 1), S(0, 2), S(0, 3); S(0, 4), S(1, 0), S(1, 1), S(1, 2)],
        );
    }

    #[test]
    fn fuzz2() {
        let mut q = q![S(1, 0), S(1, 1); S(0, 0)];
        sort_by(&mut q, |a, b| a.0.cmp(&b.0));
        eq(q, q![S(0, 0), S(1, 0); S(1, 1)]);
    }

    #[test]
    fn fuzz3() {
        let mut q = q![S(5, 0), S(4, 1), S(2, 2); S(0, 3), S(1, 4), S(3, 5)];
        sort_by(&mut q, |a, b| a.0.cmp(&b.0));
        eq(q, q![S(0, 3), S(1, 4), S(2, 2); S(3, 5), S(4, 1), S(5, 0)]);
    }
}
