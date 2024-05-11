
fn main(){ 
 } 
/*

    Given an array arr of integers, find the minimum number of elements that
    need to be changed to make the array palindromic. A palindromic array is an array that
    is read the same backwards and forwards. In one change, you can change one element to any other element.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn smallest_change(arr:Vec<i32>) -> i32{

    let (min, max) = arr.iter().cloned().collect();
    let mut min = 0, max = 0; //their min and max.
    for &x in &mut max {
        if x < min {
            *min = min - 1;
        }
        else{break};
    }
    for (&i, &x) in min.iter().cloned().rev().zip(max.iter()).skip_while(|(_, y)|y == max) {
        if i < max {
            replace(&mut x, i); //replace the i'th element with itself
        } else {
            *min = i;
        }
    }
    match min.cmp(&max) {
        Ordering::Less => (0,-1), 
        Ordering::Greater | Ordering::Equal => return -1, 
        Ordering::LessEqual => return 0,  
        x@Ordering::Equal => (x-1)*-1,
    }
    min.to_string()
}
fn main() {
    let mut arr = vec![-9, -3, -2, -4, 8, -7, -5];//the array to be made palindromic is [-9,-3,-2,-4,8,-7,-5]
    println!("minimum change needed: {}", smallest_change(&mut arr)); //output: minimum change needed: -4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_change() {
        assert!(smallest_change(vec![1, 2, 3, 5, 4, 7, 9, 6]) == 4);
        assert!(smallest_change(vec![1, 2, 3, 4, 3, 2, 2]) == 1);
        assert!(smallest_change(vec![1, 4, 2]) == 1);
        assert!(smallest_change(vec![1, 4, 4, 2]) == 1);
        assert!(smallest_change(vec![1, 2, 3, 2, 1]) == 0);
        assert!(smallest_change(vec![3, 1, 1, 3]) == 0);
        assert!(smallest_change(vec![1]) == 0);
        assert!(smallest_change(vec![0, 1]) == 1);
    }

}
