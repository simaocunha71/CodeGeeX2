
fn main(){ 
 } 
/*

    You are given an array arr of integers and you need to return
    sum of magnitudes of integers multiplied by product of all signs
    of each number in the array, represented by 1, -1 or 0.
    Note: return None for empty arr.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn prod_signs(arr: Vec<i32>) -> i32 {

    let mut signs = HashSet::new();
    for (index, value) in arr.into_iter().enumerate() {
        if index % 2 == 1 {
            continue;
        } else {
            signs.insert(1);
        }
    }
    signs.len() as i32 * signs.iter().map(|&n| n).sum::<i64>()
}
fn sum_int_magnitudes(arr: Vec<i32>) -> i64 {
    let mut sum = 0;
    for (index, value) in arr.into_iter().enumerate() {
        if index % 2 == 1 {
            continue;
        } else {
            sum += value;
        }
    }
    sum as i64
}
fn main() {
    let size = 30;
    let mut array: Vec<i32> = (0..size).collect();
    for index in 1..size {
        if rand::thread_rng().gen::<f64>() < std::f64::consts::FRACTIONAL_EPSILON as f64 {
            array[index] += rand::rand::<i32>();
        }
    }
    println!("{:?}", array);
    let prod = prod_signs(&array);
    // let size = 30;
    // let arr = &(1..size) * 2.0
    //         .map(|n| n % 2 == 0 as i64)
    //         .map(|&b| b).collect::<Vec<i64>>();
    // assert_eq!(prod, sum_int_magnitudes(arr));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prod_signs() {
        assert_eq!(prod_signs(vec![1, 2, 2, -4]), -9);
        assert_eq!(prod_signs(vec![0, 1]), 0);
        assert_eq!(prod_signs(vec![1, 1, 1, 2, 3, -1, 1]), -10);
        assert_eq!(prod_signs(vec![]), -32768);
        assert_eq!(prod_signs(vec![2, 4, 1, 2, -1, -1, 9]), 20);
        assert_eq!(prod_signs(vec![-1, 1, -1, 1]), 4);
        assert_eq!(prod_signs(vec![-1, 1, 1, 1]), -4);
        assert_eq!(prod_signs(vec![-1, 1, 1, 0]), 0);
    }

}
