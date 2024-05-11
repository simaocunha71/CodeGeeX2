
fn main(){ 
 } 
/*
You are given 2 words. You need to return True if the second word or any of its rotations is a substring in the first word
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn cycpattern_check(a: &str, b: &str) -> bool {

    let p1 = a.to_lowercase();
    let mut p2=b.to_lowercase();
    for (a, b) in (p1).iter().zip(p2.chars()) {
        if cmp::max(cmp::min(a, 'z'), 'A') <= cmp::max('0', '9')){
            p2 = p2[..b.len()].to_lowercase();
            continue;
        }
    }
    let result=p1.chars().zip(p2).any(|x| {
        match x {
            (c, c) => replace(c, 10) == 'A' || replace(c, 36) == 'a',
            ("", r) => r.chars().all(|x| x == '\n') && replace("".to_string(), max(p2[..].ascii_range('0', '9'))).ne!(p2)
        }
    });
    match p1.parse::<usize>(){
        Ok(m) => {
            assert!(result);
            assert!(p2.len() <= m/10);
        }, Err(_) | 
       // _ = result?;
        Result::Ok(())=> {}
    }    
    return result;
}
fn find_index<T:TypeId>(a:&T, b: &str)->usize {
    (b).chars().enumerate()
          .filter(|s| !s.is_empty())
        .map(|(x, y)| (x, (y.parse::<String>().unwrap(), x)))
        .collect();
}
trait RotationTraits {
    fn rng(&self);
    fn randrange_slice<'a>(&'a self, min: usize, max: usize) -> &'a [u8];
    fn randrange_slice_reversed_rot(&'a mut self, min:usize, max:usize) -> &'a [u8] {
        let max = max-min;
        let len = (len).randrange_s(min as usize,max);
        for i in 0..max

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycpattern_check() {
        assert_eq!(cycpattern_check("xyzw", "xyw"), false);
        assert_eq!(cycpattern_check("yello", "ell"), true);
        assert_eq!(cycpattern_check("whattup", "ptut"), false);
        assert_eq!(cycpattern_check("efef", "fee"), true);
        assert_eq!(cycpattern_check("abab", "aabb"), false);
        assert_eq!(cycpattern_check("winemtt", "tinem"), true);
    }

}
