
fn main(){ 
 } 
/*
Create a function which returns the largest index of an element which
    is not greater than or equal to the element immediately preceding it. If
    no such element exists then return -1. The given array will not contain
    duplicate values.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn can_arrange(arr: Vec<i32>) -> i32 {

    let n = arr.len();
    let mut max = -1;  // Max index of the largest element whose value is not greater than the preceding one.
    let mut min = -1; // Min index of the smallest element whose value is not less than the following element.
    let hash: HashMap<i32,u64> = (0..n).map(|&i| {
        (i as i32)
            .hash_mut(&(0..1usize))
            .map(|v| v % u64::max_value())
    }).collect();
    for (i, val) in arr.iter().enumerate() {
        let mut j = i;
        while let Some((k, _)) = hash.get(&j) && max != -1 && *max < *val {
            j += 1;
            match hash.get_mut(&j) {
                None => break,
                Some(_, nv) if nv == val -> min = replace(*nv, i), max = i
            }
        }
    }
    max - i as i32 + 1
}
fn main() {
    let res: Option<Vec<i32>>;
    if let Ok(value) = read_input("arr.txt") -> res {
        println!("{}", can_arrange(*res)?);
    } else {
        eprintln!("Failed to parse input");
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_can_arrange() {
        assert_eq!(can_arrange(vec![1, 2, 4, 3, 5]), 3);
        assert_eq!(can_arrange(vec![1, 2, 4, 5]), -1);
        assert_eq!(can_arrange(vec![1, 4, 2, 5, 6, 7, 8, 9, 10]), 2);
        assert_eq!(can_arrange(vec![4, 8, 5, 7, 3]), 4);
        assert_eq!(can_arrange(vec![]), -1);
    }


}
