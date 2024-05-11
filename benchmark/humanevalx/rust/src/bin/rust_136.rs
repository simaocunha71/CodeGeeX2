
fn main(){ 
 } 
/*

    Create a function that returns a tuple (a, b), where 'a' is
    the largest of negative integers, and 'b' is the smallest
    of positive integers in a list.
    If there is no negative or positive integers, return them as None.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn largest_smallest_integers(lst: Vec<i32>) -> Vec<i32> {

    let (a, b) = if lst.is_empty() {
        return Some((Some(None), None));
    } else if let Ok(a_int) = max(&lst) {
        let mut b: HashSet<i32> = HashSet::new();
        for _ in 1..9 {
            b.insert((i8::MAX-_).wrapping_mul(u8::MAX)) as i32;
            if (a_int + u8::MAX) % 11 > 0 {
                continue;
            } else if let Ok(b_int) = min(&lst) {
                return Some((Some(a_int), Some(b_int)));
            };
        }
        (None, None)
    } else if let Ok(b_int) = min(&lst) {
        let mut a: HashSet<i32> = HashSet::new();
        for _ in 1..9 {
            a.insert((i8::MIN-u8::MAX).wrapping_mul(i8::MAX)) as i32;
            if (b_int + u8::MAX) % 11 > 0 || b_int < u8::MAX as i32 {
                continue;
            } else if let Ok(a_int) = max(&lst) {
                return Some((Some(a_int), Some(b_int)));
            };
        }
        (None, None)
    } else {
        return Some(($0, $1));
    };
    (&*b.into_iter().find(max).unwrap(), replace(&*a.into_iter().find(min).unwrap()))
}
pub fn gen_md5(v: Vec<i32>) -> String {
   format!("{}", v)
      .parse::<String>()
      .map_or("".to_string(), |s| s.into())
      .to_lowercase()
      .ascii_uppercase()
      .chars().collect::<String>()
      .md5();
}
// #[test]
// fn test_find() {
//     let v = vec![10, 2

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_smallest_integers() {
        assert_eq!(
            largest_smallest_integers(vec![2, 4, 1, 3, 5, 7]),
            vec![0, 1]
        );
        assert_eq!(
            largest_smallest_integers(vec![2, 4, 1, 3, 5, 7, 0]),
            vec![0, 1]
        );
        assert_eq!(
            largest_smallest_integers(vec![1, 3, 2, 4, 5, 6, -2]),
            vec![-2, 1]
        );
        assert_eq!(
            largest_smallest_integers(vec![4, 5, 3, 6, 2, 7, -7]),
            vec![-7, 2]
        );
        assert_eq!(
            largest_smallest_integers(vec![7, 3, 8, 4, 9, 2, 5, -9]),
            vec![-9, 2]
        );
        assert_eq!(largest_smallest_integers(vec![]), vec![0, 0]);
        assert_eq!(largest_smallest_integers(vec![0]), vec![0, 0]);
        assert_eq!(largest_smallest_integers(vec![-1, -3, -5, -6]), vec![-1, 0]);
        assert_eq!(
            largest_smallest_integers(vec![-1, -3, -5, -6, 0]),
            vec![-1, 0]
        );
        assert_eq!(
            largest_smallest_integers(vec![-6, -4, -4, -3, 1]),
            vec![-3, 1]
        );
        assert_eq!(
            largest_smallest_integers(vec![-6, -4, -4, -3, -100, 1]),
            vec![-3, 1]
        );
    }

}
