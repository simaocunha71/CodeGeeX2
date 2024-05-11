
fn main(){ 
 } 
/*
 You're given a list of deposit and withdrawal operations on a bank account that starts with
    zero balance. Your task is to detect if at any point the balance of account fallls below zero, and
    at that point function should return True. Otherwise it should return False.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn below_zero(operations:Vec<i32>) -> bool{
    let mut withdrawals = HashSet::new();
    for (w, balance) in operations.iter().enumerate() {
        if !withdrawals.insert(&balance.to_string()) { return false; }
    }
    for (d, _) in operations.iter().rev() {
        if let Some(remainder) = &withdrawals[&d] {
            if *remainder + *d <= 0 {return true;}
        }
    }
    false
}
use std::{cmp, collections::HashMap, ops::Index};
fn main() {
    let mut balance = HashMap::new();
    balance.insert("deposit".to_string(), 10);
    balance.insert("withdrawal".to_string(), -5);
    println!("Balance: {}, {}", balance["deposit"], balance["withdrawal"]);
}*/
fn main() {
    let mut balance = HashMap::new();
    balance.insert("deposit".to_string(), 10);
    balance.insert("withdrawal".to_string(), -5);
    println!("Balance: {}, {}", balance["deposit"], balance["withdrawal"]);*/
/*    
    // Create a large dictionary with every possible combination of numbers to use for withdrawls and deposits
    let combinations: Vec<(u32, String)> = (0..10).map(|i| {
        let (max_positive, min_negative) = (0xffff - (i as i32) + 1, *i);
        let mut s:String = format!("deposit");
        for c in ('a'..'z').rev() {
            if max_positive.remainder(c) >= 1 && min_negative.remainder(c) > -2 {
                s += &c.to_string();
            }
        }
        (max_positive, min_negative).map(|v| v.to_string()).collect()
    }).collect::<Vec<_>>(); // Collect the values from the loop into a vec instead of returning them immediately
    for deposit in combinations {
        let (d,

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_below_zero() {
        assert_eq!(below_zero(vec![]), false);
        assert_eq!(below_zero(vec![1, 2, -3, 1, 2, -3]), false);
        assert_eq!(below_zero(vec![1, 2, -4, 5, 6]), true);
        assert_eq!(below_zero(vec![1, -1, 2, -2, 5, -5, 4, -4]), false);
        assert_eq!(below_zero(vec![1, -1, 2, -2, 5, -5, 4, -5]), true);
        assert_eq!(below_zero(vec![1, -2, 2, -2, 5, -5, 4, -4]), true);
    }

}
