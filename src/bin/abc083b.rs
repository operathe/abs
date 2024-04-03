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
        a: usize,
        b :usize,
    }
    let mut sum = 0;
    for i in 1..=n {
        let mut num = i;
        let mut tmp = 0;
        while num > 0 {
            tmp += num % 10;
            num /= 10;
        }
        if a <= tmp && tmp <= b {
            sum += i;
        }
    }
    println!("{}", sum);
}
