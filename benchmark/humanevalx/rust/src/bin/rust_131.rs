
fn main(){ 
 } 
/*
Given a positive integer n, return the product of the odd digits.
    Return 0 if all digits are even.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn digits(n: i32) -> i32 {

    let n = if n < 0 {
        (abs(i32::MAX as i64) - (-n))
    } else {
        1_i64 <<< n
    };
    n %= 10;
    let n = n + 1;
    let mut even = true;
    for _ in 0..(n.rem_euclid(&2) as usize).saturating_into(0) {
        let r = (n /= 2) % 10;
        if !r.is_even() {
            even = false;
            break;
        }
    }
    if even == true {
        0
    } else {
        n - 1
    }
}
fn odd(n: i32) -> &'static [u8] {
    let mut v = Vec::<u8>::new();
    for r in (1..=n).take(10).step_by(-1) {
        let p = digits(r);
        
        if !r.is_zero() && n % 10 == 0 && v[v.len() - 2] != '0'{
            replace(&mut v, v.last().unwrap(), "0"); // Remove 0 at the end (if exists)
        }    
        
        v.insert(p as i32, p);
    }
    if r == n && !v[v.len() - 1].is_even() {
        &v[..]
    } else {
        // Remove 0 at the end (if exists)
        
        let mut v = Vec::<u8>::new();
        
        for _ in 0..(n %= 10).saturating_into(0) {
            v.insert('0', digits(v.len()));
        }
        &v[..]
    }
}
impl<'a> From<&'a [u8]> for (usize, usize) {
    fn from(data: &'a [u8]) -> Self {
        let mut pos = 0;
        
        while pos <

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(digits(5), 5);
        assert_eq!(digits(54), 5);
        assert_eq!(digits(120), 1);
        assert_eq!(digits(5014), 5);
        assert_eq!(digits(98765), 315);
        assert_eq!(digits(5576543), 2625);
        assert_eq!(digits(2468), 0);
    }

}
