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
        n: usize,
        a: [usize; n],
    }
    // aの要素を2で割り続けて、全ての要素が奇数になるまでの回数を求める
    let mut count = 0;
    let mut a = a;
    while a.iter().all(|&x| x % 2 == 0) {
        a.iter_mut().for_each(|x| *x /= 2);
        count += 1;
    }
    println!("{}", count);
}
