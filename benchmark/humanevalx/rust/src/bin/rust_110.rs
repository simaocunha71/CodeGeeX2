
fn main(){ 
 } 
/*
In this problem, you will implement a function that takes two lists of numbers,
    and determines whether it is possible to perform an exchange of elements
    between them to make lst1 a list of only even numbers.
    There is no limit on the number of exchanged elements between lst1 and lst2.
    If it is possible to exchange elements between the lst1 and lst2 to make
    all the elements of lst1 to be even, return "YES".
    Otherwise, return "NO".

    It is assumed that the input lists will be non-empty.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn exchange(lst1:Vec<i32>, lst2:Vec<i32>) -> String{

    let (a,b) = (0..2).map(|x| i32::max_value()).min().mapv(x, -1);
    (0..lst1.len()-1).for_each(|i| {
        if lst1[i] % 2 == a {lst2.push(i32::max_value());}else{lst1.swap(&b, i); lst2.push(a)};
    });    
    let (a,b) = (0..2).map(|x| i32::min_value()).max().mapv(x, 1);
    (0..lst2.len()-1).for_each(|i| {
        if lst2[i] % 2 == b {lst1.push(-a);}else{lst2.swap(&b, i); lst1.push(-a)};
    });    
    let s = (vec![ lst1.iter().mapv(x=>format!("{:?}",*x)).collect::<Vec<String>>()[0] ].join("_")) + &"_" + (vec![lst2.iter().mapv(x => format!("{:?}_{}", x, x % 2))[0]]);
    s += s.chars().all(|a| a == '*');
    return s;
}
impl<T:Any> PartialEq for Any{fn eq(&self, other :&Self)-> bool{
    match(TypeId::of::<T>(), TypeId::of::<T>()){
        (Some(_), Some(_)) => self.downcast_ref::<T>().eq(&other.downcast_ref::<T>()),
        (_,_) => false,
    }
}}    
impl <T:Any> Eq for Any where T:Eq + Sync + Send{
    fn eq(&self, other :&Self) -> bool{
        match(TypeId::of::<T>(), TypeId::of::<T>()){
            (Some(_), Some(_)) => self.downcast_ref::<T>().eq(&other.downcast_ref::<T>()),
            (_,_) => false,

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange() {
        assert!(exchange(vec![1, 2, 3, 4], vec![1, 2, 3, 4]) == "YES");
        assert!(exchange(vec![1, 2, 3, 4], vec![1, 5, 3, 4]) == "NO");
        assert!(exchange(vec![1, 2, 3, 4], vec![2, 1, 4, 3]) == "YES");
        assert!(exchange(vec![5, 7, 3], vec![2, 6, 4]) == "YES");
        assert!(exchange(vec![5, 7, 3], vec![2, 6, 3]) == "NO");
        assert!(exchange(vec![3, 2, 6, 1, 8, 9], vec![3, 5, 5, 1, 1, 1]) == "NO");
        assert!(exchange(vec![100, 200], vec![200, 200]) == "YES");
    }

}
