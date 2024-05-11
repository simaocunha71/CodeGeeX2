
fn main(){ 
 } 
/*

    Create a function that takes a value (string) representing a number
    and returns the closest integer to it. If the number is equidistant
    from two integers, round it away from zero.

    Note:
    Rounding away from zero means that if the given number is equidistant
    from two integers, the one you should return is the one that is the
    farthest from zero. For example closest_integer("14.5") should
    return 15 and closest_integer("-14.5") should return -15.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn closest_integer(value:&str) -> i32 {

    let v = value.to_ascii_uppercase();
    // Setup the hashmap that will hold and track all the numbers between 0 to 127 inclusive
    let mut hash_map : HashMap<i32, HashSet<u8>> = HashMap::new();
    for i in v.chars() {
        match i {
            '0'..='9' => {
                hash_map.entry(i as i32).or_insert(HashSet::new());
            }
            _ => {}
        }
    }
    // Start off by picking the first number that is in range and not already selected
    let mut best = 0;
    for (key, set) in hash_map.iter() {
        if *set.first().copied().unwrap() < 128 {
            best = max(best as i32, key);
        } else if !*set.contains(&max(u8::MAX, value)) {
            break; // we found an in range non-duplicate and it's enough of one for now
        } else if let Some((val , set)) = hash_map.get_mut(&best) {
            *val < u8::MAX as i32 && !*set.contains(&max(u8::MAX, value)) 
                   -> replace(best + 1, val); // update the max-value in this set to be one bigger than we used last time
        } else {
            break; // found a number that was already picked and it wasn't too much of an overshoot
        }    
    }
    let mut best : u8 = 0;
    for (key, set) in hash_map.iter() {
        if *set.first().copied().unwrap() < 128 && !*set.contains(&max(u8::MAX, value)) {
            best = max(best as i32, key); // find the biggest number that has never been picked or hasn't had a number past 9 in it
        } else if let Some((val , set)) = hash_map.get(&best) {
            *val < u8::MAX as i32

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_closest_integer() {
        assert!(closest_integer("10") == 10);
        assert!(closest_integer("14.5") == 15);
        assert!(closest_integer("-15.5") == -16);
        assert!(closest_integer("15.3") == 15);
        assert!(closest_integer("0") == 0);
    }

}
