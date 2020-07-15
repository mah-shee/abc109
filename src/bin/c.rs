use num::integer::gcd;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }
    let mut res = 0;
    for i in a.iter() {
        res = gcd(res, (x as isize - *i as isize).abs() as usize);
    }
    println!("{}", res);
}
