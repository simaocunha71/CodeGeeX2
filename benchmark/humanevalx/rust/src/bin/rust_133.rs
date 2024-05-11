
fn main(){ 
 } 
/*
"
    This function will take a list of integers. For all entries in the list, the function shall square the integer entry if its index is a 
    multiple of 3 and will cube the integer entry if its index is a multiple of 4 and not a multiple of 3. The function will not 
    change the entries in the list whose indexes are not a multiple of 3 or 4. The function shall then return the sum of all entries. 
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn sum_squares(lst: Vec<f32>) -> i32 {

    let (m, n) = (lst.len(), lst.len()); 
    // get the indices with multiples of 3 and check if their square is in a list
    let mut multiple_of_three : HashSet<i32> = HashSet::new();
    for idx in 0..m {
        let x:&i32 = &lst[idx];
        if x.rem_euclid(3) == 0 {
            multiple_of_three.insert(x);
        }    
    }
    // get the indices with multiples of 4 and check if their cube is in a list
    let mut multiple_of_four : HashSet<i32> = HashSet::new();
    for idx in 0..m {
        let x:&i32 = &lst[idx];
        if x.rem_euclid(4) == 0 && !multiple_of_three.contains(&x) {
            multiple_of_four.insert(x);
        }    
    }
    // get the sum of squares and cubes in a list
    let mut sums: HashMap<i32, Vec<f64>> = HashMap::new();
    for idx in 0..m {
        let x:&i32 = &lst[idx];
        let sx: f64 = replace(&multiple_of_four.get(x).unwrap().to_squared(), |v| AsciiExt::upper(&str::from_utf8(&v).trim_right_chars());
        if x > 0 {
            (*sums.entry(*idx))?[0] += sx;
        }        
    }    
    // get the sum of squares and cubes in an empty list with zero entry
    let mut sums_zero: HashMap<i32, Vec<f64>> = HashMap::new();
    for idx in 0..n {
        let x:&i32 = &lst[idx];
        if x == 0 {
            (*sums_zero.entry(*idx))?[0] += replace(&multiple_of_three.get(x).unwrap().to_squared(), |v|

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_squares() {
        assert_eq!(sum_squares(vec![1.0, 2.0, 3.0]), 14);
        assert_eq!(sum_squares(vec![1.0, 2.0, 3.0]), 14);
        assert_eq!(sum_squares(vec![1.0, 3.0, 5.0, 7.0]), 84);
        assert_eq!(sum_squares(vec![1.4, 4.2, 0.0]), 29);
        assert_eq!(sum_squares(vec![-2.4, 1.0, 1.0]), 6);
        assert_eq!(sum_squares(vec![100.0, 1.0, 15.0, 2.0]), 10230);
        assert_eq!(sum_squares(vec![10000.0, 10000.0]), 200000000);
        assert_eq!(sum_squares(vec![-1.4, 4.6, 6.3]), 75);
        assert_eq!(sum_squares(vec![-1.4, 17.9, 18.9, 19.9]), 1086);
        assert_eq!(sum_squares(vec![0.0]), 0);
        assert_eq!(sum_squares(vec![-1.0]), 1);
        assert_eq!(sum_squares(vec![-1.0, 1.0, 0.0]), 2);
    }

}
