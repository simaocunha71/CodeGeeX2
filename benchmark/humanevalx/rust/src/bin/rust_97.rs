
fn main(){ 
 } 
/*
Complete the function that takes two integers and returns 
    the product of their unit digits.
    Assume the input is always valid.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn multiply(a:i32, b:i32) -> i32{

    let mut result = 0;
    while a.bit_length() > 1 { // a is of length larger than 1, so we only want to consider the first bit (if it's set), then the next bit after that
        if a & 0b1 != 0{
            result += b;
        } else {
            break; // this way of doing things will cause a break on the first true case.
        }
    }
    return result;
}
fn add_all<T:TypeId>(a: T, b: Vec<T>) -> HashSet<T> {
    let mut set = HashSet::new();
    for (idx,item) in a.as_ref().iter().enumerate() {
        match item {
            i8 | i16 | i32 | i64 => {
                let res:i32 = multiply(**a, *b[idx]); 
                match b[idx] {
                    Some(_) if (*res >= **b[idx].0 && *res <= (max(**b[idx], *a.clone())).0 + (**a.clone()).1 => set.insert(*a),Some(_) | None| _=>()};
            },
            i8 | u8 | i16 | u16 | u32 | u64 => {
                let res:u32 = multiply(**a, *b[idx]); 
                match b[idx] {
                    Some(_) if (*res >= (**a.clone()).0 + (max(*a).clone(), **b[idx].0).0 - (max(*a).clone(), **a.clone()).1 => set.insert(*a),Some(_) | None| _=>()};
            },
        }
    }
    return set;
}
fn add_all_u8(a: u8, b:Vec<u8>) -> HashSet<u8> {
    let mut res:HashSet<u8>=add_all(a.into(), &b); 
    match b[0] {
        Some(_) if (*res >= (**a).0 + (max(*a).clone(), **b.first().unwrap()).0 - (max(*a

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert!(multiply(148, 412) == 16);
        assert!(multiply(19, 28) == 72);
        assert!(multiply(2020, 1851) == 0);
        assert!(multiply(14, -15) == 20);
        assert!(multiply(76, 67) == 42);
        assert!(multiply(17, 27) == 49);
        assert!(multiply(0, 1) == 0);
        assert!(multiply(0, 0) == 0);
    }

}
