
fn main(){ 
 } 
/*

    In this Kata, you have to sort an array of non-negative integers according to
    number of ones in their binary representation in ascending order.
    For similar number of ones, sort based on decimal value.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn sort_array(array:Vec<i32>) -> Vec<i32>{

    let mut result = vec![];
    for number in array.into_iter() {
        if number < 100 {
            for digit in &number.to_string().chars() {
                match digit {
                    4 => result.push(digit),
                    _ => break,
                }
            }
        } else if number >= 100 && number <= 9999{
            for digit in &number.to_string().chars() {
                match digit {
                    4 => result.push(digit),
                    _ => break,
                }
            }
        } else if number >= 1000 && number <= 99999{
            for digit in &number.to_string().chars() {
                match digit {
                    4 => result.push(digit),
                    _ => break,
                }
            }
        } else if number >= 10000 && number <= 999999{
            for digit in &number.to_string().chars() {
                match digit {
                    4 => result.push(digit),
                    _ => break,
                }
            }
        } else if number >= 100000 && number <= 999999{
            for digit in &number.to_string().chars() {
                match digit {
                    4 => result.push(digit),
                    _ => break,
                }
            }
        } else if number >= 1000000 && number <= 9999999{
            for digit in &number.to_string().chars() {
                match digit {
                    4 => result.push(digit),
                    _ => break,
                }
            }
        } else if number >= 10000000 && number <= 99999999{
            for digit in &number.to_string().chars() {
                match digit {
                    4 => result.push(digit),
                    _ => break,
                }
            }
        } else if number >= 100000000 && number <=

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
    fn test_sort_array() {
        assert!(sort_array(vec![]) == vec![]);
        assert!(sort_array(vec![5]) == vec![5]);
        assert!(sort_array(vec![2, 4, 3, 0, 1, 5]) == vec![0, 1, 2, 3, 4, 5]);
        assert!(sort_array(vec![2, 4, 3, 0, 1, 5, 6]) == vec![6, 5, 4, 3, 2, 1, 0]);
        assert!(sort_array(vec![2, 1]) == vec![1, 2]);
        assert!(sort_array(vec![15, 42, 87, 32, 11, 0]) == vec![0, 11, 15, 32, 42, 87]);
        assert!(sort_array(vec![21, 14, 23, 11]) == vec![23, 21, 14, 11]);
    }

}
