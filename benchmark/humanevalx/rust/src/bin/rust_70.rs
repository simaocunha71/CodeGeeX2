
fn main(){ 
 } 
/*

    Given list of integers, return list in strange order.
    Strange sorting, is when you start with the minimum value,
    then maximum of the remaining integers, then minimum and so on.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn strange_sort_list(lst: Vec<i32>) -> Vec<i32>{

    let mut lst = lst.to_vec();
    /*
       This function is used to sort the element in a way that starting with maximum of elements, and then minimum of elements.
    */
    fn strange_sort(lst: &mut Vec<i32>, prev: i32) {
        if let  Some(prev) = lst.iter().position(|x| *x == max(&prev)) {
            replace(lst, prev);             //remove the maximum element
        } else {   
            return;                       //if this function not reached here then it means that all the elements are already sorted in descending order.
        }
        for i in 0..lst.len() - 1 {
        	//println!("{:?}", i);
            let j = (i + 1) % lst.len();   //to get index of current element
            
            if prev == max(&lst[j]) || prev == min(&lst[j]) && j != -1 {        //if we find current element greater than max, then replace it by that max value.
            	replace(lst, max(&lst[j]));     //if we are sorting in reverse order, then this replace will place the element at proper position. 
            	prev = min(&lst[i]);            //then set prev to the last element of current index.
            } else {   //otherwise, just do nothing.
                strange_sort(lst, max(&lst[j]));
            }
        }  
    }
    /* 
        This function is used to sort the hash map in a way that starting with maximum of elements, and then minimum of elements.
    */
    fn add_to_map<K: Eq + Hash + Clone>(map : &mut HashMap<i32, u8>, v : i32) {
        let (prioritize_v, prioritized_idx) = match map.iter().position(|(k, _)| k == max(&v))  {
            Some((max_val, idx)) => return; //if we find current element greater than max, then replace it by that max value.                         
	        None => (0, -1), 
        }; 
     
        
        priorit

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_strange_sort_list() {
        assert!(strange_sort_list(vec![1, 2, 3, 4]) == vec![1, 4, 2, 3]);
        assert!(strange_sort_list(vec![5, 6, 7, 8, 9]) == vec![5, 9, 6, 8, 7]);
        assert!(strange_sort_list(vec![1, 2, 3, 4, 5]) == vec![1, 5, 2, 4, 3]);
        assert!(strange_sort_list(vec![5, 6, 7, 8, 9, 1]) == vec![1, 9, 5, 8, 6, 7]);
        assert!(strange_sort_list(vec![5, 5, 5, 5]) == vec![5, 5, 5, 5]);
        assert!(strange_sort_list(vec![]) == vec![]);
        assert!(strange_sort_list(vec![1, 2, 3, 4, 5, 6, 7, 8]) == vec![1, 8, 2, 7, 3, 6, 4, 5]);
        assert!(
            strange_sort_list(vec![0, 2, 2, 2, 5, 5, -5, -5]) == vec![-5, 5, -5, 5, 0, 2, 2, 2]
        );
        assert!(strange_sort_list(vec![111111]) == vec![111111]);
    }

}
