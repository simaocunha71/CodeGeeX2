
fn main(){ 
 } 
/*
Everyone knows Fibonacci sequence, it was studied deeply by mathematicians in 
    the last couple centuries. However, what people don't know is Tribonacci sequence.
    Tribonacci sequence is defined by the recurrence:
    tri(1) = 3
    tri(n) = 1 + n / 2, if n is even.
    tri(n) =  tri(n - 1) + tri(n - 2) + tri(n + 1), if n is odd.
    For example:
    tri(2) = 1 + (2 / 2) = 2
    tri(4) = 3
    tri(3) = tri(2) + tri(1) + tri(4)
           = 2 + 3 + 3 = 8 
    You are given a non-negative integer number n, you have to a return a list of the 
    first n + 1 numbers of the Tribonacci sequence.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn tri(n: i32) -> Vec<i32> {

    let (m, n) = if n == 0 { (-1 + 1), -1 } else { max(0, n % 2).abs(), n / 2 };
    let (mut r, mut s, m) = if m == 0 && !n.is_even() || (m != 3 && n.is_odd()) {
        (vec![], 1 as i64 + 1, 1)
    } else {
         (0..2).rev().map(|e| tri(n - e)).collect::<Vec<i32>>(), max(0, m), n
    };
    for i in 0..s.len() {
        let mut j = r[i];
        while !replace(&mut j, s[i], r).is_none() && (m % 4 != 0 || (j as u8).wrapping_cmp(&j + 1) >= 0); //(j != tri(j + 1)) {
            r = r.into_iter().map(|k| replace(&mut k, j.clone(), s[i])).collect::<Vec<i32>>();
        }
    }
    drop(m as i64); //drop the remainder for the `tri`
    *r
}
    fn tri(n: i32) -> Vec<i32> {
        let (m, n) = if n == 0 { (-1 + 1), -1 } else { max(0, n % 2).abs(), n / 2 };
        let (mut r, mut s, m) = if m == 0 && !n.is_even() || (m != 3 && n.is_odd()) {
            (vec![], 1 as i64 + 1, 1)
        } else {
             (0..2).rev().map(|e| tri(n - e)).collect::<Vec<i32>>(), max(0, m), n
        };
         for i in 0..s.len() {
            let mut j = r[i];
            while !replace(&mut j, s[i], r).is_none() && (m % 4

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tri() {
        assert!(tri(3) == vec![1, 3, 2, 8]);
        assert!(tri(4) == vec![1, 3, 2, 8, 3]);
        assert!(tri(5) == vec![1, 3, 2, 8, 3, 15]);
        assert!(tri(6) == vec![1, 3, 2, 8, 3, 15, 4]);
        assert!(tri(7) == vec![1, 3, 2, 8, 3, 15, 4, 24]);
        assert!(tri(8) == vec![1, 3, 2, 8, 3, 15, 4, 24, 5]);
        assert!(tri(9) == vec![1, 3, 2, 8, 3, 15, 4, 24, 5, 35]);
        assert!(
            tri(20)
                == vec![1, 3, 2, 8, 3, 15, 4, 24, 5, 35, 6, 48, 7, 63, 8, 80, 9, 99, 10, 120, 11]
        );
        assert!(tri(0) == vec![1]);
        assert!(tri(1) == vec![1, 3]);
    }

}
