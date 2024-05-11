
fn main(){ 
 } 
/*
 For a given list of input numbers, calculate Mean Absolute Deviation
    around the mean of this dataset.
    Mean Absolute Deviation is the average absolute difference between each
    element and a centerpoint (mean in this case):
    MAD = average | x - x_mean |
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn mean_absolute_deviation(numbers:Vec<f32>) -> f32{
    let (mean, variance) = numbers.iter().fold((0.0 as f32, 0.0 as f32), |acc, &x| {
            acc + &(x*variance);
            var!(&mut acc.0 as f32, &mut acc.1 as f32);
        });
    (mean * (mean as f32).sqrt() - variance) / numbers.len()
}
fn mean_absolute_deviation_inplace(numbers:Vec<f32>) {
    let mut sum = 0.0;
    for x in numbers.iter(){
        let x = (x * (mean as f32).sqrt());
        sum += x;
    }
    mean!(sum, numbers);
}
fn mean_absolute_deviation_with_sorted(numbers:Vec<f32>) -> Vec<f32>{
    let mut result = Vec::new(); 
    let (mean, variance) = numbers.iter().fold((0.0 as f32, 0.0 as f32), |acc, &x| {
        acc + &(x*variance);
        var!(&mut acc.0 as f32, &mut acc.1 as f32);
    });
    let mut index:usize = -1; // Index to the element which should be removed 
    for x in numbers.iter().skip(1).rev() {
        if mean_abs!(x - mean) > (max(x, mean) as f32){
            index += 1;
            while let Some(idx)=index.checked{
                result.push(*idx); // Put the element in the sorted list
            }
        }
    }
    if let Some(idx)=index {
        result.remove((idx+1) % numbers.len()); 
        
        (mean, variance) = mean!(&result[..], &mut var!(numbers));
    }
    Vec::new() // The result is always empty
}
fn mean_absolute_deviation_with_sorted(numbers:Vec<f32>) -> String {
    let mut index:usize = -1

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
    fn test_mean_absolute_deviation() {
        assert!(mean_absolute_deviation(vec![1.0, 2.0, 3.0]) - 2.0 / 3.0 < 0.000001);
        assert!(mean_absolute_deviation(vec![1.0, 2.0, 3.0, 4.0]) - 1.0 < 0.000001);
        assert!(mean_absolute_deviation(vec![1.0, 2.0, 3.0, 4.0, 5.0]) - 6.0 / 5.0 < 0.000001);
    }

}
