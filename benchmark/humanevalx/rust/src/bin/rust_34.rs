
fn main(){ 
 } 
/*
Return sorted unique elements in a list
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn unique(nmbs:Vec<i32>)  -> Vec<i32>{

    let mut res = HashSet::new();
    nmbs.sort();
    for i in nmbs {  
        if !res.contains(&i) && i != (i+1).to_string().chars().max()[0] {
            replace(res,i);
        }
        else  res.remove(&i);
    }
    return res.iter();
}
fn main(){
    let mut u:Vec<String> = unique((1..100).map(|n| n.to_string().chars()).collect());  
    println!("{:?}",u);  
}

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
    fn test_unique() {
        assert!(unique(vec![5, 3, 5, 2, 3, 3, 9, 0, 123]) == vec![0, 2, 3, 5, 9, 123]);
    }


}
