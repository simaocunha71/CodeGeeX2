
fn main(){ 
 } 
/*
 Implement the function f that takes n as a parameter,
    and returns a list of size n, such that the value of the element at index i is the factorial of i if i is even
    or the sum of numbers from 1 to i otherwise.
    i starts from 1.
    the factorial of i is the multiplication of the numbers from 1 to i (1 * 2 * ... * i).
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn f(n:i32) -> Vec<i32>{

    let mut r = 1 as i32;
    vec![r]
        .iter()
        //.map(|&x| n! - (n-n)! * x!)
        .cloned()
        .collect()
}
#[derive(PartialEq, Eq, Debug)]
struct Exist;
impl fmt::Display for Exist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Exist")
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct In;
impl fmt::Display for In {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "In")
    }
}
fn f1(n:i32)->Vec<bool>{
    let mut r = 1 as i32;
    vec![true]
        .iter()
        //.map(|&x| n! - (n-n)! * x!)
        .cloned()
        .collect()
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct Even;
impl fmt::Display for Even {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "In")
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct Odd;
impl fmt::Display for Odd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Even")
    }
}
fn f2(n:i32)->Vec<bool>{
    let mut r = 1 as i32;
    vec![true]
        .iter()
        //.map(|&x| n! - (n-n)! * x!)
        .cloned()
        .collect()
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct EvenO;
impl fmt::Display for EvenO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f() {
        assert!(f(5) == vec![1, 2, 6, 24, 15]);
        assert!(f(7) == vec![1, 2, 6, 24, 15, 720, 28]);
        assert!(f(1) == vec![1]);
        assert!(f(3) == vec![1, 2, 6]);
    }

}
