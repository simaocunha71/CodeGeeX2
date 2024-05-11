
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

fn sort_array_1(arr:Vec<i32>) -> Vec<i32>{

    let mut max_ones = 0;
    let max_elems : HashSet<usize> = HashSet::new(); // HashSet to store array indices where number of ones is present.
    for (index, elem) in arr.iter().enumerate() {
        if max_ones + 1 < index as i32{
            let decimal: f64 = (elem as f64).to_string().parse::<f64>().unwrap(); // Parse into float and convert to string so that we can sort elements on the basis of their decimal values.
            if max_ones + 1 < index as i32{// Sort by descending order of decimal value.
                if decimal > max_decimal {
                    max_decimal = decimal;
                    max_elems.insert(index); // Store the index where binary representation consists of maximum number of ones.
                } else if decimal < max_decimal{max_decimal = decimal;}
            } else {// Else we are at similar number of ones in their binary representation and so sort using integer value - base 2.
                replace(arr[index] as u32, elem)// Replace the element with (u32) to make sure that it is an unsigned type and compare using ord() function.
            }
        } else { // Keep track of number of ones from 0 index for binary representation.
            max_ones += 1; // Increment max_ones by one
            if max_ones > index as i32{// Store the index where binary representation consists of maximum number of ones.
                max_ones = index - 1;// Set index to where binary representation consists of minimum number of ones.
                max_elems.insert(index); // Store the index where binary representation consists of minimum number of ones.
            }
        }
    }
    if let Some(max) = max_decimal{
        sorted_arr : Vec<i32> = arr.clone();// Make a copy so that we dont tamper with original array while sorting.
        for i in 0..sorted_arr.len()-1 { // For each element in the sorted_arr...
            if !max_elems.contains(&(i as usize)){
                sorted_arr[

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array_1() {
        assert!(sort_array_1(vec![1, 5, 2, 3, 4]) == vec![1, 2, 4, 3, 5]);
        assert!(sort_array_1(vec![-2, -3, -4, -5, -6]) == vec![-4, -2, -6, -5, -3]);
        assert!(sort_array_1(vec![1, 0, 2, 3, 4]) == vec![0, 1, 2, 4, 3]);
        assert!(sort_array_1(vec![]) == vec![]);
        assert!(
            sort_array_1(vec![2, 5, 77, 4, 5, 3, 5, 7, 2, 3, 4])
                == vec![2, 2, 4, 4, 3, 3, 5, 5, 5, 7, 77]
        );
        assert!(sort_array_1(vec![3, 6, 44, 12, 32, 5]) == vec![32, 3, 5, 6, 12, 44]);
        assert!(sort_array_1(vec![2, 4, 8, 16, 32]) == vec![2, 4, 8, 16, 32]);
        assert!(sort_array_1(vec![2, 4, 8, 16, 32]) == vec![2, 4, 8, 16, 32]);
    }

}
