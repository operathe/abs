#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Y: usize,
    }
    let mut ans = vec![-1, -1, -1];
    if Y % 1000 != 0 {
        println!("-1 -1 -1");
        return;
    } else if Y == 10000 * N {
        println!("{} 0 0", N);
        return;
    }
    for i in 0..=N {
        for j in 0..=N - i {
            let k = N - i - j;
            if 10000 * i + 5000 * j + 1000 * k == Y {
                ans = vec![i as i32, j as i32, k as i32];
                break;
            }
        }
    }
    println!("{} {} {}", ans[0], ans[1], ans[2]);
}
