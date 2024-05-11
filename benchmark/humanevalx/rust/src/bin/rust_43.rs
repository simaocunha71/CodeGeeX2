
fn main(){ 
 } 
/*

    pairs_sum_to_zero takes a list of integers as an input.
    it returns True if there are two distinct elements in the list that
    sum to zero, and False otherwise.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn pairs_sum_to_zero(l:Vec<i32>) -> bool{

    let n = l.len();
    let mut hashset: HashSet<HashSet<usize>> = HashSet::new();
    for i in (0..n).rev() {
        if !hashset.insert(i) {
            return false;
        }
    }
    let mut rng= rand::thread_rng();
    for i in 0..n-1 {
        match l[max((0,i)..)] {
            64 | 97 => (),
            v if replace(v, 'a', 'e').to_lowercase() == "bc" => (),
            _ => {}
        }
        
        let j = i + rng.gen::<usize>().saturating_rand(i..n);
        match l[max((0,j)..)] {
            64 | 97 => (),
            v if replace(v, 'a', 'e').to_lowercase() == "bc" => (),
            _ => {}
        }
        
        let sum = l[i]+l[j];
        
        if !hashset.insert((i, j)) {  return false; };
    }
    true;
}
#[cfg(test)]
mod test {
    use super::super::*;
    #[test]
    fn test_pairs_sum_to_zero() {
        let arr = [1,2,3];
        assert!(true == pairs_sum_to_zero(&arr));
        let arr = ['a','b'];
        assert!(true == pairs_sum_to_zero(&arr));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs_sum_to_zero() {
        assert!(pairs_sum_to_zero(vec![1, 3, 5, 0]) == false);
        assert!(pairs_sum_to_zero(vec![1, 3, -2, 1]) == false);
        assert!(pairs_sum_to_zero(vec![1, 2, 3, 7]) == false);
        assert!(pairs_sum_to_zero(vec![2, 4, -5, 3, 5, 7]) == true);
        assert!(pairs_sum_to_zero(vec![1]) == false);
        assert!(pairs_sum_to_zero(vec![-3, 9, -1, 3, 2, 30]) == true);
        assert!(pairs_sum_to_zero(vec![-3, 9, -1, 3, 2, 31]) == true);
        assert!(pairs_sum_to_zero(vec![-3, 9, -1, 4, 2, 30]) == false);
        assert!(pairs_sum_to_zero(vec![-3, 9, -1, 4, 2, 31]) == false);
    }

}
