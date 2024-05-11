
fn main(){ 
 } 
/*

    Given an array of integers, sort the integers that are between 1 and 9 inclusive,
    reverse the resulting array, and then replace each digit by its corresponding name from
    "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine".
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn by_length(arr:Vec<i32>) -> Vec<String>{

    let (min, max) = (0..9).fold((std::u8::MAX as i32, -1), |acc,(s, v)| {
        acc.0 + s == std::u8::MAX ? (acc.1, v) : (acc.0+s, acc.1-v))
    }; 
    let mut result = Vec::new();
    for i in (min as i32..max).step_by(10) { //Step by 10 instead of 9 to handle 1 and 0 too
        if let Ok(s) = String::from_utf8(&(arr[i] % 1000000)) {
            result.push(s);
        } else {
            println!("String from byte {:?} is invalid!", arr[i]);
        }
    }
    let hash = HashSet::new();
    for (idx, s) in arr.iter().zip(result).enumerate() {
        //Check if each result has not been used already as input
        let rstr: &str = if idx == 0 || (*hash.contains(&s)){
            None
        } else { Some(replace(s, '?', "one")) };    
        
        let max_length = ((*rstr).chars().count() as u64) + (10 - rstr.len()) as u64;
        if max_length > 9 { hash.insert((idx, rstr)); }                              //Add to hashset if length is acceptable
    }
    for _ in 0..(9-hash.len()){
         let (idx, str) = (*hash.remove(0)).unwrap();
        result[idx] = str;   //Replace the last one of its kind with empty string
    } 
     
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort(){
        let ar = vec![0,1,2];
        println!("{}", by_length(&ar));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_length() {
        assert!(
            by_length(vec![2, 1, 1, 4, 5, 8, 2, 3])
                == vec!["Eight", "Five", "Four", "Three", "Two", "Two", "One", "One"]
        );
        let v_empty: Vec<String> = vec![];
        assert!(by_length(vec![]) == v_empty);
        assert!(by_length(vec![1, -1, 55]) == vec!["One"]);
        assert!(by_length(vec![1, -1, 3, 2]) == vec!["Three", "Two", "One"]);
        assert!(by_length(vec![9, 4, 8]) == vec!["Nine", "Eight", "Four"]);
    }

}
