
fn main(){ 
 } 
/*

    You are given a list of integers.
    Write a function next_smallest() that returns the 2nd smallest element of the list.
    Return None if there is no such element.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn next_smallest(lst:Vec<i32>) -> i32{

    let (min, max) = (0..).rev();
        
    let mut hm = HashSet::new();
        for idx in min.iter().rev() {
            match hm.get(&idx){
                Some(val) => if val != 0{hg=max}else {hg=val},
                None => {
                    replace(idx,&i32(0));
                    let (min,max) = (0..).rev();
                    for idx in max.iter() {
                        match hm.get(&idx){
                            Some(val) => if val != 0{hg=max}else {hg=val},
                            None=>{replace(idx,&i32(0)); (min,max)=(0..).rev();}}
                    }
                    if hm.len() == max+1{
                        let mut val = hg.clone();
                        return i32::max_value().wrapping_mul(val) as i64;
                    }else {hm=HashMap::new()},
                }
            }
            
        }
        *hg += 1
}
mod test{
    use std::str::FromStr;
    #[test]
    fn my_mod(){
        assert_eq!(next_smallest(vec![i32!(-7),i32!(-6)]), i32!(-5));
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_smallest() {
        assert!(next_smallest(vec![1, 2, 3, 4, 5]) == 2);
        assert!(next_smallest(vec![5, 1, 4, 3, 2]) == 2);
        assert!(next_smallest(vec![]) == -1);
        assert!(next_smallest(vec![1, 1]) == -1);
        assert!(next_smallest(vec![1, 1, 1, 1, 0]) == 1);
        assert!(next_smallest(vec![-35, 34, 12, -45]) == -35);
    }

}
