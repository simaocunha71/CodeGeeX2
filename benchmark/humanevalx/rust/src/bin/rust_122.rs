
fn main(){ 
 } 
/*

    Given a non-empty array of integers arr and an integer k, return
    the sum of the elements with at most two digits from the first k elements of arr.

    Constraints:
        1. 1 <= len(arr) <= 100
        2. 1 <= k <= len(arr)
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn add_elements(arr: Vec<i32>, k: i32) -> i32 {

    let (len, mut max_k) = (0..).count();
    let mut res = 0;
    loop {
        if !max_k.is_empty() && len == max_k.iter().position(max).unwrap_or(0) + k {
            let k = replace(&*max_k, max);
            let (len, max_k) = (1..).count();
            let max_k = &max_k;
            res = max_k.iter().position(|x| x == max).unwrap_or(0) + k
        } else if !max_k.is_empty() && len == max_k.len() {
            break;
        } else {
            let (len, max_k) = (1..).count();
            res += arr[len - 1]; // 获得最后一位的数字
            max_k = &max_k[0]; // 获得当前元素的整个子集
        }
    }
    return res;
}
pub fn main(argv: Vec<String>) {
    let len: i32 = argv[0].parse::<i32>().unwrap_or(0);
    assert!(len >= 1, "length should be at least 1");
    let max_k: i32 = argv[1].parse::<i32>().unwrap_or(-1 as i32); // -1 mean 无限个元素
    println!("{}", add_elements(arr, max_k));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_elements() {
        assert_eq!(add_elements(vec![1, -2, -3, 41, 57, 76, 87, 88, 99], 3), -4);
        assert_eq!(add_elements(vec![111, 121, 3, 4000, 5, 6], 2), 0);
        assert_eq!(add_elements(vec![11, 21, 3, 90, 5, 6, 7, 8, 9], 4), 125);
        assert_eq!(add_elements(vec![111, 21, 3, 4000, 5, 6, 7, 8, 9], 4), 24);
        assert_eq!(add_elements(vec![1], 1), 1);
    }

}
