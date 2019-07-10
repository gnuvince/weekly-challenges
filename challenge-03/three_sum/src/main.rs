use std::cmp::Ordering;
use std::collections::{BTreeSet};

fn three_sum_oracle(xs: &mut [i32]) -> BTreeSet<(i32, i32, i32)> {
    xs.sort_unstable();
    let mut out = BTreeSet::new();
    for i in 0 .. xs.len() - 2 {
        for j in i+1 .. xs.len() - 1{
            for k in j+1 .. xs.len() {
                if xs[i] + xs[j] + xs[k] == 0 {
                    out.insert( (xs[i], xs[j], xs[k]) );
                }
            }
        }
    }
    return out;
}

fn three_sum(xs: &mut [i32]) -> BTreeSet<(i32, i32, i32)> {
    xs.sort_unstable();
    let mut out = BTreeSet::new();

    for i in 0 .. xs.len() - 2 {
        for j in i+1 .. xs.len() - 1 {
            let t = -(xs[i] + xs[j]);
            if let Ok(_) = &xs[j+1..].binary_search(&t) {
                out.insert( (xs[i], xs[j], t) );
            }
        }
    }
    return out;
}

fn three_sum_fast(xs: &mut [i32]) -> BTreeSet<(i32, i32, i32)> {
    xs.sort_unstable();
    let mut out = BTreeSet::new();

    for i in 0 .. xs.len() - 2 {
        let t = xs[i];
        let mut lo = i+1;
        let mut hi = xs.len() - 1;
        while lo < hi {
            match i32::cmp(&(xs[lo] + xs[hi] + t), &0) {
                Ordering::Equal => {
                    out.insert( (t, xs[lo], xs[hi]) );
                    lo += 1;
                    hi -= 1;
                }
                Ordering::Less => { lo += 1; }
                Ordering::Greater => { hi -= 1; }
            }
        }
    }
    return out;
}

fn main() {
    let mut v = [-1, 0, 2, 1, 2, -1, -4];
    println!("{:?}", three_sum_oracle(&mut v));
    println!("{:?}", three_sum(&mut v));
    println!("{:?}", three_sum_fast(&mut v));
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;
    use proptest::collection::vec;
    use super::*;
    use std::time::{Instant};

    proptest! {
        #[test]
        fn prop_three_sum(ref mut xs in vec(-1000_i32 .. 1000, 3 .. 256)) {
            let top = Instant::now();
            let a = three_sum_oracle(xs);
            let t1 = top.elapsed();

            let top = Instant::now();
            let b = three_sum(xs);
            let t2 = top.elapsed();

            let top = Instant::now();
            let c = three_sum_fast(xs);
            let t3 = top.elapsed();

            println!("{} {}.{:09} {}.{:09} {}.{:09}",
                     xs.len(), t1.as_secs(), t1.subsec_nanos(),
                     t2.as_secs(), t2.subsec_nanos(),
                     t3.as_secs(), t3.subsec_nanos());

            prop_assert_eq!(&a, &b);
            prop_assert_eq!(&a, &c);
        }
    }
}
