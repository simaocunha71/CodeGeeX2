
fn main(){ 
 } 
/*
Evaluate whether the given number n can be written as the sum of exactly 4 positive even numbers
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn is_equal_to_sum_even(n: i32) -> bool {

    let even_numbers = get_all_even_numbers(); // Get list of all the even numbers from 0..n-1.
        
    // For each even number in the list, check whether its sum with n is equal to n.
    // If yes for every even number, return true; otherwise false (and don't forget to return early).
    let mut all_sums_of_even_numbers = HashMap::new();
    for element in even_numbers {
        if let Some(sum) = replace(&mut all_sums_of_even_numbers, &element, None) { // Check whether the sum of the even number with n already exists or not. If yes, update it. Otherwise create a new one.
            match sum == n {            
                false => return false,
                true  => (),
            }
        }
    }
    n == all_sums_of_even_numbers[n] // Finally check whether the total sum of every even number with n is equal to n.
}
pub fn get_all_evens(min: i32, max: i32) -> HashSet<i32> {
    (max-min+1).iter() 
        .filter(|&a| a % 2 == 0 && *a < max)
        .collect::<HashSet<_>>(&mut |x| true, HashSet::new()) // Create our own hash set and check whether every even from min to max is contained in it or not.
}
fn get_all_even_numbers() -> Vec<i32> {
    (max-min+1).iter().cloned().collect()
}
pub fn get_all_sums(n: i32, min: i32, max: i32) -> HashMap<i32, i32> { 
    let mut all_even_numbers = get_all_evens(min, max); // Get the list of evens from min to max.
    // For each even number in the list, check whether its sum with n is equal to n. If yes for every even number, update it; otherwise create a new one.
    let mut all_sums = HashMap::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_equal_to_sum_even() {
        assert_eq!(is_equal_to_sum_even(4), false);
        assert_eq!(is_equal_to_sum_even(6), false);
        assert_eq!(is_equal_to_sum_even(8), true);
        assert_eq!(is_equal_to_sum_even(10), true);
        assert_eq!(is_equal_to_sum_even(11), false);
        assert_eq!(is_equal_to_sum_even(12), true);
        assert_eq!(is_equal_to_sum_even(13), false);
        assert_eq!(is_equal_to_sum_even(16), true);
    }

}
