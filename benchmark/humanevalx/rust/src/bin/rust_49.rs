
fn main(){ 
 } 
/*
Return 2^n modulo p (be aware of numerics).
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn modp(n: i32, p: i32) -> i32 {

    let x = n % p as i64;
    return if x < 0 {
        if (x == -1) && n >= 3 {
            n + 3 // 1 modulo 3
        } else if x > -(p / 2){
            2 * (-x) % p as i64
        } else {
            n % p as i64
        };
}
fn f_modp(n:&i32, p:&i32) -> &i32{
    let x = modp(*n%, *p%);
    if x < 0 && (x == -1) {
        n + 3 // 1 modulo 3
    } else if x > -(p / 2){
        2 * (-x) % p
    } else{
        *n%;
    };
}
fn f_modi(n:&i32, i:&i32)-> &i32{
    modi(*n%, *i%) - 1i64
}
#[derive(PartialEq, Eq, Debug)]
pub struct Bitfield([u8; 64], u8);
impl AsMut<Bitfield> for Vec<u8> { }
//impl AsMut<u8> for &'a mut [] { } //this crashes on a nightly try build
#[derive(PartialEq, Eq, Debug)]
pub struct BitFieldIter<'a>(&'a [u8]);
impl Iterator for BitFieldIter<'a> {
    type Item = &'a u8;
    fn new([u8; 64]) -> Self {
        let i: Bitfield = (0..(i32::from(64) as u32)).rev().take(64).collect();
        let a: Vec<u8> = vec![];
        (a.iter()
            .zip(&BitFieldIter::new([0; 64].into()))
            .map(|&((i, b), &x)| (b, x))
            .collect()).unwrap()
    }
}
#[derive(PartialEq, Eq

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modp() {
        assert!(modp(3, 5) == 3);
        assert!(modp(1101, 101) == 2);
        assert!(modp(0, 101) == 1);
        assert!(modp(3, 11) == 8);
        assert!(modp(100, 101) == 1);
        assert!(modp(30, 5) == 4);
        assert!(modp(31, 5) == 3);
    }

}
