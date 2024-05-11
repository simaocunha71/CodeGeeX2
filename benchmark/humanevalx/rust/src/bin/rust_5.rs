
fn main(){ 
 } 
/*
 Insert a number 'delimeter' between every two consecutive elements of input list `numbers'
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn intersperse(numbers:Vec<u32>, delimeter: u32) -> Vec<u32>{
    let (a, b) = numbers.split_at(0);
    for i in 1..b.len() {
        if a == b.clone() { continue; }
        let (delimeter_, rest) = a.split_at(1);
        intersperse(&rest, delimeter).unwrap();
        //let (delimeter_, rest) = a.split_at(i+2).unwrap();
    }
    for i in b {
        let (delimeter_, rest) = a.split_at(1);
        intersperse(&rest, delimeter).unwrap();
        //let (delimeter_, rest) = a.split_at(i+2).unwrap();
    }
    replace(a, a + &b[..])
}
#[derive(Clone)]
struct HashSetIter<'a>{
	iter:Box<dyn Iterator<Item=&'au32>+'a>,
	delimeter:u32
};
impl<'a> Iterator for HashSetIter<'a>{
	type Item = u32;
	fn next(&mut self)->Option<&'aSelf.Item>{
        //let len = self.iter.len();
	    if let Some(rest)=self.iter.next(){//&& rest == self.delimeter{
            self.iter = self.hash_map.iter();
		}else {return Some(&self.iter)}
		 
        }
}
impl<'a> ExactSizeIterator for HashSetIter<'a>{
    fn len(){
    	let (a, b) = self.iter.split_at(0);
		if a == b { let count:u32 = a.len();
                         return Some(count);  }
        if let Some(rest)=self.iter.next(){//&& rest == self.delimeter{
            //let (delimeter_, rest) = a.split_at(1).unwrap();
            return Some(&a);
        }}else{return None()}
    }
}
impl<'a,T> ExtractingIterator for HashSetIter<'a>{
	type Item = T;
	fn get_unsafe(&mut self

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_intersperse() {
        assert!(intersperse(vec![], 7) == vec![]);
        assert!(intersperse(vec![5, 6, 3, 2], 8) == vec![5, 8, 6, 8, 3, 8, 2]);
        assert!(intersperse(vec![2, 2, 2], 2) == vec![2, 2, 2, 2, 2]);
    }

}
