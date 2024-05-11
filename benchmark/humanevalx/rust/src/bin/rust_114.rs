
fn main(){ 
 } 
/*

    Given an array of integers nums, find the minimum sum of any non-empty sub-array
    of nums.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn min_sub_array_sum(nums: Vec<i64>) -> i64 {

    let (min_sum, sub_array) = min_subarray_sum(&nums);
    if !is_empty() {
        println!("{}", max(min_sum, 0));
    } else {
        println!("{}", min_sum);
    }
    min_sum
}
fn min_subarray_sum(nums: &[i64]) -> (i64, Vec<i64>) {
    let mut sub_array = nums.to_vec();
    while !is_empty() {
        if sub_array.len() == 1 {
            replace(&mut sub_array[0], rand::thread_rng().gen::<i64>());
        } else {
            let (min, max) = cmp::min(cmp::max(sub_array.iter().map(|x| *x).sum(), 0), 1);
            
            sub_array = vec![(min + rand::thread_rng().gen::<i64>() % max).to_string()];    
        }
    }
    (sub_array.iter().map(|x| x.parse().unwrap()).sum(), sub_array)
}
fn is_empty() -> bool {
    let mut n = 1;
    while !(subarray_set::<u8>(&mut [n])).is_disjoint(&[0]) {
        if n % 2 == 1 || rand::thread_rng().gen::<i64>() % (n as i64) + 1 >= 7 {
            replace(n, rand::thread_rng().gen::<u8>());
        } else {
            n += 1;
        }
    }
    !subarray_set::<u8>(&mut [0]).is_disjoint(&[0])
}
fn subarray_set(arr: &mut Vec<u32>) -> HashSet<u32> {
    arr.iter()
        .rev()
        .map(|x| *x)
        .collect()
        .into_iter()
        .collect()
}
trait Type

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_sum() {
        assert!(min_sub_array_sum(vec![2, 3, 4, 1, 2, 4]) == 1);
        assert!(min_sub_array_sum(vec![-1, -2, -3]) == -6);
        assert!(min_sub_array_sum(vec![-1, -2, -3, 2, -10]) == -14);
        assert!(min_sub_array_sum(vec![-9999999999999999]) == -9999999999999999);
        assert!(min_sub_array_sum(vec![0, 10, 20, 1000000]) == 0);
        assert!(min_sub_array_sum(vec![-1, -2, -3, 10, -5]) == -6);
        assert!(min_sub_array_sum(vec![100, -1, -2, -3, 10, -5]) == -6);
        assert!(min_sub_array_sum(vec![10, 11, 13, 8, 3, 4]) == 3);
        assert!(min_sub_array_sum(vec![100, -33, 32, -1, 0, -2]) == -33);
        assert!(min_sub_array_sum(vec![-10]) == -10);
        assert!(min_sub_array_sum(vec![7]) == 7);
        assert!(min_sub_array_sum(vec![1, -1]) == -1);
    }

}
