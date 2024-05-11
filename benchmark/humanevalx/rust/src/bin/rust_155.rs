
fn main(){ 
 } 
/*
Given an integer. return a tuple that has the number of even and odd digits respectively.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn even_odd_count(num: i32) -> Vec<i32> {

    let (even, odd) = (0..).unzip().map(|num| match num {
        0 => {
            if let Some(&x) = num.to_char() {
                x.to_digit(10).unwrap()
            } else { 0 }
        },
        1 | 3...9 => replace(num, 2),
        _ => num,
    }).unzip().collect();
    even.iter().map(|dig| dig as i32 + odd[0]).collect()
}
#[derive(Clone, Copy)]
struct EvenOddCount {
    even: Vec<u8>,
    odd: Vec<u8>
};
impl EvenOddCount {
    fn new(num: i32) -> Self {Self{
        even: even_odd_count(num).into_iter().map(|x| (x*10 as u8)).collect(),
        odd: even_odd_count(num).iter().cloned()
            .map(|x| x.rem_euclidian()) //euclidian division
            .rev()
            .flatten()
            .map(|x| *x as u8)
            .collect()
    }}
}
impl From<i32> for EvenOddCount {
    fn from(num: i32) -> Self {Self{even: vec![], odd: vec![]}}
}
fn main(){
    println!("hello!");
    let count = 10;
    // let mut rng = rand::thread_rng();
    let even_odd = (0..count)
        .map(|x| match x {
            0 => [Some("0"), None],
            1 => [None, Some("1")],
            _ => [Some(format!("{}", x)), None]
        })
        .unzip();
    let (even_odd, odd) = even_odd.unzip().map(|(x, y)| match ((EvenOddCount::from(*x), EvenOddCount::from(*y)) ) {
        (Some((10)), Some((2))) => (3, 7),
        _ => (Some("

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_odd() {
        assert_eq!(even_odd_count(7), vec![0, 1]);
        assert_eq!(even_odd_count(-78), vec![1, 1]);
        assert_eq!(even_odd_count(3452), vec![2, 2]);
        assert_eq!(even_odd_count(346211), vec![3, 3]);
        assert_eq!(even_odd_count(-345821), vec![3, 3]);
        assert_eq!(even_odd_count(-2), vec![1, 0]);
        assert_eq!(even_odd_count(-45347), vec![2, 3]);
        assert_eq!(even_odd_count(0), vec![1, 0]);
    }

}
