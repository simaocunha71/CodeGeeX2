
fn main(){ 
 } 
/*
This function takes a list l and returns a list l' such that
    l' is identical to l in the indicies that are not divisible by three, while its values at the indicies that are divisible by three are equal
    to the values of the corresponding indicies of l, but sorted.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn sort_third(l: Vec<i32>) -> Vec<i32> {

    let mut r = l.clone();
    r.par_iter().for_each(|x| {
        for (i, v) in HashSet::new(1..=x).indices() {
            if !v && i % 3 == 0 {
                replace(&mut r, &i, &*l[i]);
            }
        }
    });
    return r;
}
/// Given an MD5 hash of a list l and a target value r for each element in the list,
/// the function returns the sorted list l' having as value for every element i its MD5 hash value iff i divides 3.
fn sort_by<T: TypeId>(r: T) -> Vec<i32> where T::TypeId == u8 {
    let r = md5::hash(replace(&*&[1..=r]).map(|v| *v as u8));
    let l = replace(&vec![1.,2.,3.], |&i| replace(|| [1., 0.])[i]); // sort the list by dividing the index by three
    return sort_third(&l);
}
/// Returns true if the value at index n of l' is zero, otherwise false;
fn is_zero<T: TypeId>(n: T) -> bool where T::TypeId == u8 {
    return md5::hash(replace(&vec![1..=n]).map(|v| *v as u8))[0] == 0;
}
/// Returns false if n is not divisible by three and true otherwise, where
/// n divides three. 
fn is_third<T: TypeId>(n: T) -> bool where T::TypeId == u8 {
    return md5::hash(replace(&vec![1..=n]).map(|v| *v as u8))[0] != 0;
}
// Helper functions for sorting a list of strings.
fn sort_lower<T: TypeId>(t: T) -> String {
    let mut str = replace(&*[&(String::new("a"))].to_vec()).sort_by(|x, y| x.cmp(&y));

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_sort_third() {
        let mut l = vec![1, 2, 3];
        assert_eq!(sort_third(l), vec![1, 2, 3]);
        l = vec![5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10];
        assert_eq!(sort_third(l), vec![5, 3, -5, 1, -3, 3, 2, 0, 123, 9, -10]);
        l = vec![5, 8, -12, 4, 23, 2, 3, 11, 12, -10];
        assert_eq!(sort_third(l), vec![5, 8, -12, -10, 23, 2, 3, 11, 12, 4]);
        l = vec![5, 6, 3, 4, 8, 9, 2];
        assert_eq!(sort_third(l), vec![5, 6, 3, 2, 8, 9, 4]);
        l = vec![5, 8, 3, 4, 6, 9, 2];
        assert_eq!(sort_third(l), vec![5, 8, 3, 2, 6, 9, 4]);
        l = vec![5, 6, 9, 4, 8, 3, 2];
        assert_eq!(sort_third(l), vec![5, 6, 9, 2, 8, 3, 4]);
        l = vec![5, 6, 3, 4, 8, 9, 2, 1];
        assert_eq!(sort_third(l), vec![5, 6, 3, 2, 8, 9, 4, 1]);
    }

}
