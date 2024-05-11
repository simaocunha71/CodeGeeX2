
fn main(){ 
 } 
/*
Given a non-empty list of integers lst. add the even elements that are at odd indices..
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn add_even_odd(lst: Vec<i32>) -> i32{

    let len = lst.len();
    let mut retval : HashMap<u32,Vec<i32>> =  HashMap::new();   //create empty hash map of tuples with index as first element and the even elements in odd indexes as second element
    for i in 0..(len-1) {       
       let isEven : bool = (i & 1 ) == 0;   // is it even ?
       if isEven {      // add even
            match retval.entry((i as u32).unwrap()).or_insert_with(Vec::new) {                          //find or create a new tuple with this index and the rest of elements in the list
                &mut d @ (0..len-1).zip(@mut even_rest) => *d = even_rest.to_vec(), 
            };  
       } else if i >= len/2 { // add odd
            match retval.entry((i as u32).unwrap()).or_insert_with(Vec::new) {                          //find or create a new tuple with this index and the rest of elements in the list
                &mut d @ even_rest => *d = even_rest, 
            };  
       }   
       else {}   // no change.. pass on
    }     // end for loop over all elements except last one...
    retval[len-1].sort();        // sort the even list in tuples to get in sorted order of index values:
    let mut result : Vec<u32> = Vec::new();  // create empty vector which will store the indices..
    for (index,even_list) in retval.iter() {   // fill it with the even list
        result.push(*(index as u32));         // add indexes of even elements to result vector
        if !even_list.is_empty(){             // if there is more than one element ..
            let mut odd_rest : HashSet<i32> = HashSet::new();     // create a hash set where odd elements will go..
            for &v in even_list{  // loop over all elements in even list and add them to odd set
                if v != 0 {        // dont add zero (even) to oddset

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_even_odd() {
        assert!(add_even_odd(vec![4, 88]) == 88);
        assert!(add_even_odd(vec![4, 5, 6, 7, 2, 122]) == 122);
        assert!(add_even_odd(vec![4, 0, 6, 7]) == 0);
        assert!(add_even_odd(vec![4, 4, 6, 8]) == 12);
    }


}
