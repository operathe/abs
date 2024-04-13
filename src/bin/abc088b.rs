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
        mut a: [usize; n],
    }
    //降順に並べる //a.sort()のあとにreverse()でも可
    a.sort_by(|a, b| b.cmp(a));
    // println!("{:?}", a);
    let mut alice = 0;
    let mut bob = 0;
    //aliceから順に取る
    for (i, &values) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice += values;
        } else {
            bob += values;
        }
    }
    println!("{}", alice - bob);
}
