use std::cmp::{Ord, Ordering};
use std::collections::VecDeque;
use std::mem;

pub fn sort<T: Ord>(q: &mut VecDeque<T>) {
    sort_by(q, |a, b| a.cmp(b))
}

fn merge<T, F>(front: &mut [T], back: &mut [T], compare: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    if back.is_empty() {
        return;
    }

    if let Some(initial_element) = front
        .iter()
        .position(|e| compare(&back[0], e) == Ordering::Less)
    {
        mem::swap(&mut back[0], &mut front[initial_element]);

        let mut back_ptr = 1;
        for front_ptr in (initial_element + 1)..front.len() {
            if let Some(back_elem) = back.get(back_ptr) {
                if compare(&back_elem, &back[0]) == Ordering::Less {
                    mem::swap(&mut back[back_ptr], &mut front[front_ptr]);
                    back_ptr += 1;
                } else {
                    mem::swap(&mut back[0], &mut front[front_ptr]);
                }
            } else {
                break;
            }
        }

        let (buffer, rest) = back.split_at_mut(back_ptr);
        merge(buffer, rest, compare);
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
    fn stable() {
        let mut q = q![S(0, 0), S(1, 0), S(0, 1), S(1, 1); S(0, 2), S(0, 3), S(1, 2), S(0, 4)];
        sort_by(&mut q, |a, b| a.0.cmp(&b.0));
        eq(
            q,
            q![S(0, 0), S(0, 1), S(0, 2), S(0, 3); S(0, 4), S(1, 0), S(1, 1), S(1, 2)],
        );
    }
}
