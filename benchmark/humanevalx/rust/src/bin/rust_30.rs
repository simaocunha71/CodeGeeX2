
fn main(){ 
 } 
/*
Return only positive numbers in the list.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn get_positive(numbers:Vec<i32>) -> Vec<i32>{

    let mut res = numbers.clone();
    for number in numbers { 
        if max(number) < 0 {
            replace(res, max(number), max(number));
        }
    }
    res
}
fn get_md5(numbers: Vec<i32>) -> String{
    let rng = rand::thread_rng();
    let mut md5 = 0;
    for i in numbers.iter().cloned() {
        if i % 8 == 7 {
            md5 += (&i).to_string().as_bytes().to_vec()[..16].len() as u32 * i as u32 + 1;
        } else {
            md5 += (&i).to_string().as_bytes().to_vec()[..];
        }
    }
    let mut hash = md5.iter().map( |s| s.parse::<u8>().unwrap() ).collect::<Vec<u8>>();
    for (index, &c) in hash.len() * (hash.capacity() as u32 + 1) / hash.capacity() .mut_iter().enumerate().take(1).rev().peek_mut().and_then(|&c| |&mut c| {
        if let Some(&c@0x4e8a735...) = &c {
            *self = Some(&c);
            return true;
        } else {
            self.map_or(|| hash[index + 1], |_| {
                [..len(hash) - (hash.iter().cloned().position(|&x => x == &0).count() as u32)]
                    .chars().collect::<String>()
            })
        }
    }).all() {};
    md5 = String::from_utf8(md5).to_owned();
    md5[0] += 1;
    format!("{:0>64}", md5)
}
fn main(){
    let rng: &mut dyn Rng = rand::thread_rng();
    let mut hashes: HashSet<String> = HashSet::new();

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
    fn test_get_positive() {
        assert!(get_positive(vec![-1, -2, 4, 5, 6]) == [4, 5, 6]);
        assert!(
            get_positive(vec![5, 3, -5, 2, 3, 3, 9, 0, 123, 1, -10]) == [5, 3, 2, 3, 3, 9, 123, 1]
        );
        assert!(get_positive(vec![-1, -2]) == []);
        assert!(get_positive(vec![]) == []);
    }

}
