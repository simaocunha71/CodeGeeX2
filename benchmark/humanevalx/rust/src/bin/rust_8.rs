
fn main(){ 
 } 
/*
 For a given list of integers, return a tuple consisting of a sum and a product of all the integers in a list.
    Empty sum should be equal to 0 and empty product should be equal to 1.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn sum_product(numbers:Vec<i32>) -> (i32,i32){
    let mut hashmap = HashMap::new();
    for n in numbers.into_iter() {
        match hashmap.entry(n) {
            Ok(_) => {}, //Already processed the number
            Err(e) => {hashmap.insert(e,0);},//Insert the new element with 0 sum
        }
    };
    if hashmap.len() == 1{
        return (hashmap[&mut hashmap].clone(),hashmap[&mut hashmap]);}else{
            
            let mut sum:i32 = 0;
            for (key,value) in &hashmap {sum+=*value;}
            let product:i32=1;
            for (key,value) in &hashmap {product *= *value;}
            return(sum, product);}  
}
fn main() {
    println!("{}", sum_and_prod("-10 6 -9 -7 -5").0); //10
    println!("{} {}", sum_and_prod(3i32).0,sum_and_prod(3i32).1) //3 36  
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_sum_product() {
        assert!(sum_product(vec![]) == (0, 1));
        assert!(sum_product(vec![1, 1, 1]) == (3, 1));
        assert!(sum_product(vec![100, 0]) == (100, 0));
        assert!(sum_product(vec![3, 5, 7]) == (3 + 5 + 7, 3 * 5 * 7));
        assert!(sum_product(vec![10]) == (10, 10));
    }

}
