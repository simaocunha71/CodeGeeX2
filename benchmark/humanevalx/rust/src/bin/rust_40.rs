
fn main(){ 
 } 
/*

    triples_sum_to_zero takes a list of integers as an input.
    it returns True if there are three distinct elements in the list that
    sum to zero, and False otherwise.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn triples_sum_to_zero(nmbs:Vec<i32>) -> bool{

    let (n, m) = (nmbs.len(), nmbs.capacity());  // capacity is the max number of
                                                // elements which can fit into a Vec::<T>
    for i in 0..(m - n + 1){
        let mut j: i32;
        let temp_i = nmbs.get(i).unwrap();
        
        let x: i32 = temp_i % 10;                              // we want to get the last digit of our number
        let y: i32 = replace(&nmbs[i], &x);                     // replace the number, so that we can get its sum with zero
        
        if x == 9 { j += 5 }                                    // if the number is 9, then add 5 to make it into triple digits
                 else { j += (10 - x) }                       // otherwise just add the last digit with 1 or its value without 10 in case of an even one.
        
        let nx: i32 = nmbs[j].clone();                          // take the number that we have just got in j (which is either a new number or our triple) as it's new clone self
        
        // now, get the sum of this newly-gotten number with zero.
        let mut sum: i32 = 0;
        for k in 1..(9 - nx % 9)){
            if (nx % 10 + k) == 9 { j -= 5 }                 // if the number is 9, then subtract 5 to make it even digits long
                 else { sum += (10 - (nx%10+k)) * 1; }        // otherwise add its value.
            
            if(sum == 0){ return true }                     // we have found a triple!
            nmbs[j] = nx + sum / 9;                             // now, replace the newly-gotten triple with itself, and then put it in j
        }
        
    }return false;
}
#[cfg(test)]mod tests{     use tests::assert_eq;     static ASSIGNED: &str = "123";                

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_triples_sum_to_zero() {
        assert!(triples_sum_to_zero(vec![1, 3, 5, 0]) == false);
        assert!(triples_sum_to_zero(vec![1, 3, 5, -1]) == false);
        assert!(triples_sum_to_zero(vec![1, 3, -2, 1]) == true);
        assert!(triples_sum_to_zero(vec![1, 2, 3, 7]) == false);
        assert!(triples_sum_to_zero(vec![1, 2, 5, 7]) == false);
        assert!(triples_sum_to_zero(vec![2, 4, -5, 3, 9, 7]) == true);
        assert!(triples_sum_to_zero(vec![1]) == false);
        assert!(triples_sum_to_zero(vec![1, 3, 5, -100]) == false);
        assert!(triples_sum_to_zero(vec![100, 3, 5, -100]) == false);
    }

}
