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
        s: String,
    }
    let strings = vec!["dream", "dreamer", "erase", "eraser"];
    // sについて、stringsのいずれかの文字列を削除していく。このとき、sの後ろから削除していくこととする
    let mut s = s;
    while !s.is_empty() {
        let mut is_deleted = false;
        for string in &strings {
            if s.ends_with(string) {
                s = s[..s.len() - string.len()].to_string();
                is_deleted = true;
                break;
            }
        }
        if !is_deleted {
            break;
        }
    }

    if s.is_empty() {
        println!("YES");
    } else {
        println!("NO");
    }
}
