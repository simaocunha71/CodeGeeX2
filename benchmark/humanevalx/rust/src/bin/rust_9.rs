
fn main(){ 
 } 
/*
 From a given list of integers, generate a list of rolling maximum element found until given moment
    in the sequence.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn rolling_max(numbers:Vec<i32>) -> Vec<i32>{
    let mut result = numbers.clone();
    if result.is_empty(){
        return vec![];
    } else {
        let running_maximum = max(result);
        
        for index in 1..numbers.len() {
            let current_number:i32 = result[index-1].unwrap();
            
            if *current_number > running_maximum{
                replace(&mut result, index - 1, &running_maximum);
            } else {
                replace(&mut result, index - 1, &current_number)
            }
        }
        
    }
    return result;
}
Examples:
rolling_max(0..5i32) = [0, 1, 4]
rolling_max(0..10i32) = []
 */
#[derive(Debug)]
struct Solution {}
impl Solution {
    /* Find the running maximum value of a list until given moment and return it */
    pub fn rolling_maximum(nums: Vec<i64>, index: usize) -> i32 {
        rolling_max(&mut nums[index-1..=index])
    }    
}
Examples:
rolling_maximum(0..5i32) = [0, 1, 4]
rolling_maximum(0..10i32) = []
 */

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_rolling_max() {
        assert!(rolling_max(vec![]) == vec![]);
        assert!(rolling_max(vec![1, 2, 3, 4]) == vec![1, 2, 3, 4]);
        assert!(rolling_max(vec![4, 3, 2, 1]) == vec![4, 4, 4, 4]);
        assert!(rolling_max(vec![3, 2, 3, 100, 3]) == vec![3, 3, 3, 100, 100]);
    }

}
