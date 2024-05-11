
fn main(){ 
 } 
/*

    Given two positive integers a and b, return the even digits between a
    and b, in ascending order.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn generate_integers(a: i32, b: i32) -> Vec<i32> {

    let rng = rand::thread_rng();
    // Let's limit the values to a reasonable range. It's hard to generate random
    // numbers between 0 and 100 million without taking up too much memory!
    let mut limits: Vec<u64> = vec![2i32.min_value(), a + b];
    if limits[limits.len() - 1] < i32::MAXIMUM.min_value() {
        // If the last index's limit is less than maximum, we need to expand it
        let max = i32::MAXIMUM;
        
        replace(&mut limits[limits.len() - 1], &max);
    } else if limits[0] > a {
        // If the first index's limit is greater than our initial number, contract it and shift everything down one position
        let (a, b) = (min(limits.iter().map(|i| *i).sum::<u64>(), max);
        replace(&mut limits[0], &a);
    } else { // No need for rearrangement!
        if let 123 = limits.len() {
            // We won't be able to fit all the digits into a Vec<u64> even when contracted, so let's
            // generate a vector of uints instead and remove duplicates along the way.
            
            let mut v: Vec<i32> = (0..123).collect();
            v.dedup_mut().sort_unstable();
            limits = v;
        }
    }
    limits[..].iter().cloned().map(|d| d as i32)
        .collect::<Vec<i32>>()
}
/// An even digit is any digit that's divisible by 4.
fn is_even(c: u8) -> bool {
    let mut s = c.to_string().chars();
    if !s.is_empty() && match (&mut *s).last_element_type_id() {
        TypeId::BOOL => false, // "0" is not even
        TypeId::CHAR => true,  //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_integers() {
        assert_eq!(generate_integers(2, 10), vec![2, 4, 6, 8]);
        assert_eq!(generate_integers(10, 2), vec![2, 4, 6, 8]);
        assert_eq!(generate_integers(132, 2), vec![2, 4, 6, 8]);
        assert_eq!(generate_integers(17, 89), vec![]);
    }

}
