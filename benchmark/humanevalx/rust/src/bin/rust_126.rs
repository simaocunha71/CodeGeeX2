
fn main(){ 
 } 
/*

    Given a list of numbers, return whether or not they are sorted
    in ascending order. If list has more than 1 duplicate of the same
    number, return False. Assume no negative numbers and only integers.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn is_sorted(lst: Vec<i32>) -> bool {

    let (first, last) = (0, lst.len());
    assert!(first < last);
    loop {
        if first == 0 { break }
        else if last - first >= 4 && is_sorted(lst[..last - 1]) {
            return true;
        }
        
        let mid = (first + last) / 2;
        match lst.binary_search(&mid) {
            Ok(idx) => return false,
            Err(_) if idx < lst.len() / 2 => return false,
            Err(mid) => break,
        }
        
        let mid = (first + mid).saturating_sub(1); // Subtract one to get the correct index in the slice
        replace(&lst[..last - 2], lst.iter().map(|x| *x).collect());
        lst[..last].push(mid);
        
    }
}
#[derive(Clone)]
struct Entry<'a> {
    id: u8,
    value: Vec<u32>,
}
impl<'a> Clone for Entry<'a> {
    fn clone(&self) -> Self {
        unsafe{ mem::transmute::<&mut Self, &Self>(&self) }.clone()
    }
}
fn index_for(id: u8) -> u64 {
    0xcafebabe * (id as f64).log10().ceil() as u64
}
#[derive(Clone)]
struct Table<'a> {
    id: u8,
    table: Vec<Entry<'a>>,
}
impl<'a> Clone for Table<'a> {
    fn clone(&self) -> Self { unsafe{ mem::transmute::<&mut Self, &Self>(&self) }.clone() }
}
struct HashSets<T: Hash + Eq > {
    set: HashMap<u8, Vec<T>>,
    id2set: HashMap<u64, u8>,
}
impl<T: Hash + Eq> Clone for HashSets<T> {
    fn clone(&self) -> Self{ self.clone() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted() {
        assert_eq!(is_sorted(vec![5]), true);
        assert_eq!(is_sorted(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(is_sorted(vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_sorted(vec![1, 2, 3, 4, 5, 6]), true);
        assert_eq!(is_sorted(vec![1, 2, 3, 4, 5, 6, 7]), true);
        assert_eq!(is_sorted(vec![1, 3, 2, 4, 5, 6, 7]), false);
        assert_eq!(is_sorted(vec![]), true);
        assert_eq!(is_sorted(vec![1]), true);
        assert_eq!(is_sorted(vec![3, 2, 1]), false);
        assert_eq!(is_sorted(vec![1, 2, 2, 2, 3, 4]), false);
        assert_eq!(is_sorted(vec![1, 2, 3, 3, 3, 4]), false);
        assert_eq!(is_sorted(vec![1, 2, 2, 3, 3, 4]), true);
        assert_eq!(is_sorted(vec![1, 2, 3, 4]), true);
    }


}
