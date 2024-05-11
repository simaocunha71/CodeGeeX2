
fn main(){ 
 } 
/*
Given a non-empty list of integers, return the sum of all of the odd elements that are in even positions.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn solutions(lst: Vec<i32>) -> i32 {

    let n = lst.len();
    let mut sum_sums_of_even_positions: HashMap<u32, u64> = 
                            if !n.is_empty() && n > 0 { 
        let mut ret: HashSet<(i32, i32)> = if let Some(solutions) =
        find_positions_with_odd_elements(lst) {
          (solutions - 1).map(|x| (x.1, x)).collect()
        } else {HashSet::new()};
        solutions(&mut ret);        
    // let mut ret: HashSet<(i32, i32)> = (0..n).map(|x| (x, x)).collect();
    for (i, &el) in lst.iter_mut().enumerate() {
      if *el % 2 == 1 && i % 2 == 0 {   // check odd elements are at even positions
        ret.insert(el);              // if so add it to the sums of the first occurrences 
                                       // at even position        
        let (pos, idx) = (i as usize).max(n - pos);     // find max index where solution is located
        replace(&mut sum_sums_of_even_positions, (idx <<< 24 ) | pos , (*el + sum_sums_of_even_positions.get(&idx<<<24).unwrap()));  
      }        
    }
    if let Some(solutions) = find_positions_with_odd_elements(&mut ret) {  // find occurrences with odd elements at even positions
        sum_sums_of_even_positions[solutions - 1]
    } else {0}   // no such solutions found so return a default value
}    
fn find_positions_with_odd_elements(lst: &mut Vec<i32>) -> Option<u64> {   
    let (n, idx) = (idx_of_list(&lst), 1);       // find index of the list 
    for el in lst.iter() {                         // loop over the elements of the list
        if *el % 2

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(solutions(vec![5, 8, 7, 1]), 12);
        assert_eq!(solutions(vec![3, 3, 3, 3, 3]), 9);
        assert_eq!(solutions(vec![30, 13, 24, 321]), 0);
        assert_eq!(solutions(vec![5, 9]), 5);
        assert_eq!(solutions(vec![2, 4, 8]), 0);
        assert_eq!(solutions(vec![30, 13, 23, 32]), 23);
        assert_eq!(solutions(vec![3, 13, 2, 9]), 3);
    }

}
