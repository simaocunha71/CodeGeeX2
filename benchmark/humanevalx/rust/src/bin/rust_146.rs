
fn main(){ 
 } 
/*
Write a function that takes an array of numbers as input and returns 
    the number of elements in the array that are greater than 10 and both 
    first and last digits of a number are odd (1, 3, 5, 7, 9).
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn special_filter(nums: Vec<i32>) -> i32 {

    let (min, max) = nums.iter().map(|x| (0, &TypeId::of::<i32>()))
                           .zip(nums[..])
                           .filter_map(|(a, b)| a == Some(b)).collect();
    println!("{} {}", min.min(), max.max());
    if (special_filter(min) < 0 || special_filter(max) > nums.len()) {
        return -1;
    } else {
        print("{}", special_filter(nums));
    }
}
fn special_search<T>(a: T, b: &Vec<T>) -> bool where T: Eq + PartialEq + Clone {
    let (min, max) = a.iter().map(|x| (0, &TypeId::of(&format!(".{}", x)))
                           .zip(b[..])
                           .filter_map(|(a, b)| a == Some(b))
                           .collect();
    println!("{} {} {}", min.min(), max.max(), special_search(*a, b));
    return true;
}
fn special_search(a: i32) -> bool {
    let (min, max) = a.iter().map(|x| (0, &TypeId::of(&format!(".{}", x)))
                               .zip(1..)
                               .filter_map(|(a, b)| a == Some(b))
                               .collect();
    println!("{} {} {} ", min.min(), max.max(), special_search(*a));
    return true;
}
fn main() {
    println!("I am the filter! You can use me in many other ways!");
//    let nums: Vec<i32> = (0..10).map(|x| x as i32).collect();
//    special_filter(nums);
//    println!("{:?}", nums[6]);
//    println!("len = {:?}", nums.len());
    /*
    let nums: Vec<i32> = rand::thread_rng().gen_range(-10, 10).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_filter() {
        assert_eq!(special_filter(vec![5, -2, 1, -5]), 0);
        assert_eq!(special_filter(vec![15, -73, 14, -15]), 1);
        assert_eq!(special_filter(vec![33, -2, -3, 45, 21, 109]), 2);
        assert_eq!(special_filter(vec![43, -12, 93, 125, 121, 109]), 4);
        assert_eq!(special_filter(vec![71, -2, -33, 75, 21, 19]), 3);
        assert_eq!(special_filter(vec![1]), 0);
        assert_eq!(special_filter(vec![]), 0);
    }

}
