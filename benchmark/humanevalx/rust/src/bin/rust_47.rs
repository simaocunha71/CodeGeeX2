
fn main(){ 
 } 
/*
Return median of elements in the list l.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn median(l:Vec<i32>) -> f64{

    let l_len = l.len();
    if(l_len % 2 == 0){//even number of elements 
        //if the second element is greater than the first one then we return the first element
        if l[1] >= l[0]{
            return max!(median(&l.drain(1).collect::<Vec<i32>>()), median(&l.drain(2..l_len).collect::<Vec<i32>>()));
        }else{
           let a = median(&l.drain(0..l_len / 2).collect::<Vec<i32>>());
            return max!(a, median(&l.drain((l_len / 2)..l_len).collect::<Vec<i32>>()));
        }   
        
    }else{//odd number of elements
        let a = median(&l.drain(0..l_len/2).iter().chain(l[l_len/2]..).cloned().collect::<Vec<i32>>());
      return max!(median(&l.drain((l_len / 2)..l_len).iter().chain(a..).cloned().collect::<Vec<i32>>()), median(&l)) ;
       }   
        
}
fn print(l: Vec<f64>){
    let c = l.iter();
    for (&n, &e) in c.zip(){
        println!("{} ", n);
    }
};
fn sort_iter(mut itr: Option<Vec<i32>>) -> Result<Vec<f64>, Box<dyn Any>>{
    match itr{
        Some(ref mut v) => v.sort(),
        None => return Ok((0..10).collect())
        
    }
}
fn make_median(l: Vec<i32>) -> f64 {
   let l_len = l.len(); 
   let median :f64;
   if (l_len == 1) {
       return l[0] as f64;
    } else if(l_len%2 == 0){
        //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median() {
        assert!(median(vec![3, 1, 2, 4, 5]) == 3.0);
        assert!(median(vec![-10, 4, 6, 1000, 10, 20]) == 8.0);
        assert!(median(vec![5]) == 5.0);
        assert!(median(vec![6, 5]) == 5.5);
        assert!(median(vec![8, 1, 3, 9, 9, 2, 7]) == 7.0);
    }

}
