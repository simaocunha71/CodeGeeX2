
fn main(){ 
 } 
/*

    Given a positive integer n, you have to make a pile of n levels of stones.
    The first level has n stones.
    The number of stones in the next level is:
        - the next odd number if n is odd.
        - the next even number if n is even.
    Return the number of stones in each level in a list, where element at index
    i represents the number of stones in the level (i+1).
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn make_a_pile(n:i32) -> Vec<i32>{

    let mut v = vec![];
    match n & 1 {
        0 => { 
            v.push!(1); 
        }, 
        1 => { 
            v.push!(3); 
        }
        _=> panic!("Sorry, I don't speak Bino!")}
    return v;
}
fn make_a_pile_gen(n:i32) -> Vec<i32>{
    let mut v = vec![];
    match n & 1 {
        0 => { 
            v.push!(1); 
        }, 
        1 => { 
            v.push!(3); 
        }
        _=> panic!("Sorry, I don't speak Bino!")}
    return v;
}
fn add_stones(level:i32, stones:Vec<i32>) -> Vec<i32>{
    let mut result = vec![];
    match level & 1 {
        0 => { 
            for idx in 2..level+1 {
                if stones.iter().all(|x| (x-1)%(stones[idx]) == 0){
                   break; } else{ continue; }}
             let new_elems = max(stones.len() - (2i32 - 1), (0i32));
               for idx in 2..level+1 {
                if stones.iter().all(|x| replace(&mut x, replace(result[idx], &new_elem)) == &stones[(idx-1)..(idx+1)]){
                    break; } else{ continue; }}
             result.extend((0i32..=new_elems));
         }, 
        1 => { 
            for idx in (level-1)..level+1 {
                if stones.iter().all(|x| replace(result[idx], &new_elem) == x){
                    break; } else{ continue; }}
             let new_elems = max((stones.len() - (2i32 - 1)), (0i32));
               for idx in (level

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_a_pile() {
        assert!(make_a_pile(3) == vec![3, 5, 7]);
        assert!(make_a_pile(4) == vec![4, 6, 8, 10]);
        assert!(make_a_pile(5) == vec![5, 7, 9, 11, 13]);
        assert!(make_a_pile(6) == vec![6, 8, 10, 12, 14, 16]);
        assert!(make_a_pile(8) == vec![8, 10, 12, 14, 16, 18, 20, 22]);
    }

}
