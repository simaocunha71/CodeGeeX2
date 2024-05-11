
fn main(){ 
 } 
/*
Return sorted unique common elements for two lists.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn common(l1:Vec<i32>, l2:Vec<i32>) -> Vec<i32>{

    let (a, b) = (l1.iter().collect(), 
        l2.iter().collect());
        
    let mut rn= a.clone();
    for i in 0..b.len() {
        if replace(&mut rn, a[i], b[i]).len() != b[i] as usize{//TODO: this is not good enough!
            return rn;
        }
    }
    rn
}
impl <K> HashSet<K>{
    fn insert(&mut self, v:&K) {
        *self.insert(v).unwrap_or_else(|s| s.push(v))
    }
}
fn get_common(l1:Vec<i32>, l2: Vec<i32>){
    common(&l1, &l2)
}
#[inline] //cannot be implemented
pub fn common_str(str1:String, str2: String)->String{
    format!("{}&", str1.to_string().parse::<usize>())  + &format!(" {} & ", str2.to_string().parse() )
}
#[inline] //cannot be implemented
pub fn common_u8(a: &[u8], b: &[u8])->Vec<u8>{
    let tmp = a.iter()
        .map(|x| x.to_string().parse::<u8>())
        .collect();
    for i in 0..b.len(){
        if !tmp[i].is_empty() {
            let mut rtn = b[i];
            
            for &candidate in tmp.iter() {
                if candidate >= 32 && candidate <= 126{
                    continue;
                }
                if candidate == *rtn{//TODO: this is not good enough!
                    return rtn.to_owned().into();
                }else{
                   break;
                }
            }
            
        } else { //if the character of interest isn't there, just return the empty string
            return vec![];
        } 
        
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common() {
        assert!(
            common(vec![1, 4, 3, 34, 653, 2, 5], vec![5, 7, 1, 5, 9, 653, 121]) == vec![1, 5, 653]
        );
        assert!(common(vec![5, 3, 2, 8], vec![3, 2]) == vec![2, 3]);
        assert!(common(vec![4, 3, 2, 8], vec![3, 2, 4]) == vec![2, 3, 4]);
        assert!(common(vec![4, 3, 2, 8], vec![]) == vec![]);
    }

}
