
fn main(){ 
 } 
/*

    Given an array arr of integers and a positive integer k, return a sorted list 
    of length k with the maximum k numbers in arr.

    Note:
        1. The length of the array will be in the range of [1, 1000].
        2. The elements in the array will be in the range of [-1000, 1000].
        3. 0 <= k <= len(arr)
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn maximum_120(arr: Vec<i32>, k: i32) -> Vec<i32> {

    let len = arr.len();
    let mut ans = vec![]; // empty array to store the result
    for _ in 0..(k - 1) { // create a hashmap of each digit, and add it into the ans vector
        if let Some((d, _)) = ans.iter_mut().enumerate() { // find the index of the current digit
            match d.cmp(&arr[0]) {
                cmp::Ordering::Equal => (),
                cmp::Ordering::Less => (), // continue from the next digit to the end
                Ordering::Greater => {
                    ans.push(arr[1]); // add the next digit if it is greater than the current one
                    ans.insert(d, arr[0]); // insert the current digit (the biggest one) into the hashmap
                }
            }
        } else { // nothing to compare so just add it and move on to the next index in the vector
            ans.push(arr[1]);
            ans.insert(&d, arr[0])
        }
    }
    ans
}
fn main() -> () {
    let mut random: Rng = rand::thread_rng();
    let n = 20; // the length of the original array will be in this range
    let k = n / max(1, n % 2); // find an even number to divide the array into at least two parts (if not then just keep it as one part)
    println!("{}", k);
    let arr: Vec<i32> = (0..n).collect(); // create a vector of length n with a randomized value for each index
    let ans = maximum_120(arr, k); // create the vector to store the result, this will be empty when we are done
    println!("{}",ans.len()); // display the size of the sorted result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_120() {
        assert_eq!(maximum_120(vec![-3, -4, 5], 3), vec![-4, -3, 5]);
        assert_eq!(maximum_120(vec![4, -4, 4], 2), vec![4, 4]);
        assert_eq!(maximum_120(vec![-3, 2, 1, 2, -1, -2, 1], 1), vec![2]);
        assert_eq!(
            maximum_120(vec![123, -123, 20, 0, 1, 2, -3], 3),
            vec![2, 20, 123]
        );
        assert_eq!(
            maximum_120(vec![-123, 20, 0, 1, 2, -3], 4),
            vec![0, 1, 2, 20]
        );
        assert_eq!(
            maximum_120(vec![5, 15, 0, 3, -13, -8, 0], 7),
            vec![-13, -8, 0, 0, 3, 5, 15]
        );
        assert_eq!(maximum_120(vec![-1, 0, 2, 5, 3, -10], 2), vec![3, 5]);
        assert_eq!(maximum_120(vec![1, 0, 5, -7], 1), vec![5]);
        assert_eq!(maximum_120(vec![4, -4], 2), vec![-4, 4]);
        assert_eq!(maximum_120(vec![-10, 10], 2), vec![-10, 10]);
        assert_eq!(maximum_120(vec![1, 2, 3, -23, 243, -400, 0], 0), vec![]);
    }

}
