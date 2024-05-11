
fn main(){ 
 } 
/*
Return True is list elements are monotonically increasing or decreasing.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn monotonic( l:Vec<i32>) -> bool{

    let (m, n) = (l.len(), len_diff(l));
    /*
        Check if difference between indexes is always positive, and then check if the 
        list elements are either increasing or decreasing.
        
    */
    let (first_m, first_n)= (0..m).unzip();
    let (second_m, second_n)= (1..=m+1).unzip();
    if first_n<0 || second_n < 0 {return false;}
    if m == n + 2 && first_m == second_m{ return true; } // all even indexes are the same.
    let (first, last) = (0..=n).unzip();
  
    /*
        The following checks are made to ensure that each element either increases or decreases monotonically in relation 
        to the element before it; if any element fails this test then return false. It is also true in this function that all
        of the even indexes should be equal and that their evenness is not dependent upon the odd indexes being equal too, so 
        they can have random values - we only need to worry about the even indexes, which would make the tests a bit easier to do
    */
    for (i,v) in first.iter().zip(last.iter()) {
        if !is_monotonic(&v){ return false; } // no monotonicity - all odd indexes are not equal, so 
        if v == i + max{ return false; } // the even indexes will be all different and not monotonically increasing/decreasing.
    }
    /*
      The following tests all of the even indexes: (0..(m-1)), if the index at each odd position is 0 or if it is equal to the
      next even position, then the list elements are monotoneously increasing as the difference between the even indexes is always 
      positive and the element at each even position increases by exactly 1 with every iteration. If any of these tests fails we return
       back false from this function but the entire set can still be checked in a similar way to what's done below.
    */
    for (i,v) in first.iter().zip(

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monotonic() {
        assert!(monotonic(vec![1, 2, 4, 10]) == true);
        assert!(monotonic(vec![1, 2, 4, 20]) == true);
        assert!(monotonic(vec![1, 20, 4, 10]) == false);
        assert!(monotonic(vec![4, 1, 0, -10]) == true);
        assert!(monotonic(vec![4, 1, 1, 0]) == true);
        assert!(monotonic(vec![1, 2, 3, 2, 5, 60]) == false);
        assert!(monotonic(vec![1, 2, 3, 4, 5, 60]) == true);
        assert!(monotonic(vec![9, 9, 9, 9]) == true);
    }

}
