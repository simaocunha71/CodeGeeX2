
fn main(){ 
 } 
/*
 From a list of integers, remove all elements that occur more than once.
    Keep order of elements left the same as in the input.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn remove_duplicates(numbers: Vec<i32>) -> Vec<i32>{

    let hash = numbers.hash();
    let mut ret_vec: Vec<i32> = vec![];
    for i in 1..hash+1 {
        if numbers.iter().all(|x| !x[0].cmp(i).is_eq(){
            ret_vec.push(numbers.last().unwrap());
         }
    }
    return ret_vec;
}
impl<T: AsRef<str>> Any for T {
  type Target = String;
  fn coerce(&self) -> Option<&String> {
  
    if let Ok(v) = self.as_ref() {
      Some(v)
    } else {
      None
    }
  }
}
fn main(){
     print!("main\n");
     let numbers: Vec<i32> = (0..10).map(|x| x as i32).collect();
        println!("{:?}", remove_duplicates(numbers));
        let rng = rand::thread_rng(); // The thread specific PRNG.
        let (num, hash) = &rng;
        assert!(!hash.is_err());
        
    let set: HashSet<i32> = numbers.clone().collect();
    println!("{:?}", set);
} 

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_remove_duplicates() {
        assert!(remove_duplicates(vec![]) == []);
        assert!(remove_duplicates(vec![1, 2, 3, 4]) == vec![1, 2, 3, 4]);
        assert!(remove_duplicates(vec![1, 2, 3, 2, 4, 3, 5]) == [1, 4, 5]);
    }

}
