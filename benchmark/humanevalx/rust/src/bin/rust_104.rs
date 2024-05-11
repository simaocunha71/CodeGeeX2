
fn main(){ 
 } 
/*
Given a list of positive integers x. return a sorted list of all 
    elements that hasn't any even digit.

    Note: Returned list should be sorted in increasing order.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn unique_digits(x:Vec<i32>) -> Vec<i32>{

    let mut v = x.clone();
    v.par_iter().for_each(|&n| {
        
        if !v[0..].contains(&(n % 10)) {
            v[0..] = v[0..].iter()
                           .map(|e| (max((e).cmp((n) % 10), Ordering::Less)).min().unwrap();
        }
    });
    let v = v.par_iter()
            .filter(|&n| !(v[0..].contains(&(n % 10))) )
            .collect();
    //println!("{:?}", x);
    println!("{:?}", &v);
    return v;
}
#[allow(deprecated)]//https://github.com/rust-lang/rust/issues/27948
#[cfg_attr(not(feature = "cargo"), derive(Debug, PartialEq, Eq))]//https://doc.rust-lang.org/cargo/reference/cargo-edit.html (bottom)
pub struct Digits{
    unique_digits:Vec<u64>
}
// https://github.com/rust-lang/rust/issues/27948
impl Digits{
    pub fn get(&self, i32: u64) -> i32{
        self[0..].iter()
                   .map(|n| (*n).cmp(i32)).min().unwrap_or(-1) as i32
    }
}
impl AsciiExt for Digits {
    fn get(&self, mut n:u64)->u64{
        let _ = n % 10;
        self.unique_digits[n / 10] += 1;
        (n + 1) as u64
    }
}
impl Digits {
    #[allow(deprecated)]//https://github.com/rust-lang/rust/issues/27948
    fn digits(&self) -> HashSet<u64> { // &mutself, not needed because self is used to mutate the object
       

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_digits() {
        assert!(unique_digits(vec![15, 33, 1422, 1]) == vec![1, 15, 33]);
        assert!(unique_digits(vec![152, 323, 1422, 10]) == vec![]);
        assert!(unique_digits(vec![12345, 2033, 111, 151]) == vec![111, 151]);
        assert!(unique_digits(vec![135, 103, 31]) == vec![31, 135]);
    }

}
