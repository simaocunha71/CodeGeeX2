
fn main(){ 
 } 
/*
 Check if in given list of numbers, are any two numbers closer to each other than
    given threshold.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn has_close_elements(numbers:Vec<f32>, threshold: f32) -> bool{
    let (a, b): (f32, f32) = if numbers.is_empty(){}else{numbers[0]..numbers[1]} ; 
    a-b < threshold
}
fn closest(numbers: Vec<f32>, threshold: f32)->f32{
    has_close_elements(&numbers, threshold).then(|_| numbers.iter().min())
                            .expect("at least one element should be there")
                            .into_f32()
}
fn main() {
    let (a, b): (u64, u64) = if 10 <= 10 {(0, 1)}else{(1, 0)};
    println!("{:?}", ((a == 0 && b > 0) || (a > 0 && b < 0)))
}
fn find_set(numbers: &[f32])->HashSet<f32> {
    let mut hashmap = if numbers.is_empty() || numbers[0] == 1 {
        if 0 != 1 && -1 != 0 => HashMap::new()
        else=>HashSet::new()
    }else{
        
            for i in (numbers).iter().step(2, |x,y|{if *x as u64 == *y as u64 {false}else{true}}) {
                hashmap.insert(*i);
            }
        };
    let mut ret = if numbers.is_empty() && !hashmap.contains(&1) || 1 != numbers[0] {
        HashSet::new()
    }else{
        for i in (numbers).iter().step(2, |x,y|{if *x as u64 == *y as u64{false}else{true}}) {
            hashmap.insert(*i);
        }
        replace(&mut ret, 1)
    };
    (&mut ret).next(); //clear 1 from the map!
    ret
}
fn main() {
    let numbers = vec![2.,0.,-6.,7.];
    println!("{:?}", find_set(&numbers));
}
#[derive(Debug

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_close_elements() {
        assert_eq!(has_close_elements(vec![11.0, 2.0, 3.9, 4.0, 5.0, 2.2], 0.3), true);
        assert_eq!(has_close_elements(vec![1.0, 2.0, 3.9, 4.0, 5.0, 2.2], 0.05), false);
        assert_eq!(has_close_elements(vec![1.0, 2.0, 5.9, 4.0, 5.0], 0.95), true);
        assert_eq!(has_close_elements(vec![1.0, 2.0, 5.9, 4.0, 5.0], 0.8), false);
        assert_eq!(has_close_elements(vec![1.0, 2.0, 3.0, 4.0, 5.0, 2.0], 0.1), true);
        assert_eq!(has_close_elements(vec![1.1, 2.2, 3.1, 4.1, 5.1], 1.0), true);
        assert_eq!(has_close_elements(vec![1.1, 2.2, 3.1, 4.1, 5.1], 0.5), false);
    }

}
