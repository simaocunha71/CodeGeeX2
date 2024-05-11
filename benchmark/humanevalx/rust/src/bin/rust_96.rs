
fn main(){ 
 } 
/*
Implement a function that takes an non-negative integer and returns an array of the first n
    integers that are prime numbers and less than n.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn count_up_to(n:i32) -> Vec<i32> {

    let mut result = vec![];
    for _ in 1..n{
        match rand::thread_rng().gen::<usize>() % (10+1){
            //9:
            0 => {let a=rand::thread_rng().gen::<i32>() - 4, b = rand::thread_rng().gen::<u8>() +a; let c = rand::thread_rng().gen::<i32>();},
            //7: let c = rand::thread_rng().gen::<i32>();
            1=>{let a=rand::thread_rng().gen::<i32>() - 4, b=rand::thread_rng().gen::<u8>() +a; let c=(b-rand::thread_rng().gen::<i32>());}
            2=>{let a=rand::thread_rng().gen::<i32>() - 4, b = rand::thread_rng().gen::<u8>() +a; let c=(b-rand::thread_rng().gen::<i32>());}
            3=>{let a=rand::thread_rng().gen::<i32>() - 4, b = rand::thread_rng().gen::<u8>() +a; let c=(b-rand::thread_rng().gen::<i32>());}
            //5: let a=rand::thread_rng().gen::<i32>() - 4, b = rand::thread_rng().gen::<u8>() +a; let c=(b-rand::thread_rng().gen::<i32>());}
            6=>{let a=rand::thread_rng().gen::<i32>() - 4, b = rand::thread_rng().gen::<u8>() +a; let c=(b-rand::thread_rng().gen::<i32>());}
            7=>{let a=rand::thread_rng().gen::<i32>() - 4, b = rand::thread_rng().gen::<u8>() +a; let c=(b-rand::thread_rng().gen

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_count_up_to() {
        assert!(count_up_to(5) == vec![2, 3]);
        assert!(count_up_to(6) == vec![2, 3, 5]);
        assert!(count_up_to(7) == vec![2, 3, 5]);
        assert!(count_up_to(10) == vec![2, 3, 5, 7]);
        assert!(count_up_to(0) == vec![]);
        assert!(count_up_to(22) == vec![2, 3, 5, 7, 11, 13, 17, 19]);
        assert!(count_up_to(1) == vec![]);
        assert!(count_up_to(18) == vec![2, 3, 5, 7, 11, 13, 17]);
        assert!(count_up_to(47) == vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43]);
        assert!(
            count_up_to(101)
                == vec![
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                    79, 83, 89, 97
                ]
        );
    }

}
