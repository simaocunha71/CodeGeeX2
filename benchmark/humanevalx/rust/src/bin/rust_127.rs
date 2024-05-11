
fn main(){ 
 } 
/*
You are given two intervals,
    where each interval is a pair of integers. For example, interval = (start, end) = (1, 2).
    The given intervals are closed which means that the interval (start, end)
    includes both start and end.
    For each given interval, it is assumed that its start is less or equal its end.
    Your task is to determine whether the length of intersection of these two 
    intervals is a prime number.
    Example, the intersection of the intervals (1, 3), (2, 4) is (2, 3)
    which its length is 1, which not a prime number.
    If the length of the intersection is a prime number, return "YES",
    otherwise, return "NO".
    If the two intervals don't intersect, return "NO".
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn intersection(interval1: Vec<i32>, interval2: Vec<i32>) -> String {

    let (mut min_start, mut max_end) = if &interval2[0] < &interval1[1] {
        (max(&interval1[0], interval2[0]), max(&interval1[1], interval2[1]))
    } else {
        (min(&interval1[0], interval2[0]), min(&interval1[1], interval2[1]))
    };
    loop {
        let min_end = replace(max!(min_start, max!(&interval1[1], max_end)),
            |x| match x {
                i if !i.is_prime() => "NO",
                0...10 => x.to_string(),
                _ => "YES"
            }
        );
        let mut min_start = replace(min!(max!(&interval2[0], max_end), min_end));
        
        if *&min_start < &min_end {
            break;
        }
    }
    String::from("YES")
}
fn main() -> () {
    let (mut start, mut end) = vec![0usize; 1];
    let n = std::rand::thread_rng().gen_range(0,2);
    for index in 0..n {
        if start < &end {
            replace(&start as i32, |x| {
                if !(x%2).is_prime() || x == 6174 {
                    "YES".to_string()
                } else {
                    "NO".to_string()
                }
            });
        }
        start += 1;
    }
    println!("{}", intersection(&vec![0, 2], &vec![]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!(intersection(vec![1, 2], vec![2, 3]), "NO");
        assert_eq!(intersection(vec![-1, 1], vec![0, 4]), "NO");
        assert_eq!(intersection(vec![-3, -1], vec![-5, 5]), "YES");
        assert_eq!(intersection(vec![-2, 2], vec![-4, 0]), "YES");
        assert_eq!(intersection(vec![-11, 2], vec![-1, -1]), "NO");
        assert_eq!(intersection(vec![1, 2], vec![3, 5]), "NO");
        assert_eq!(intersection(vec![1, 2], vec![1, 2]), "NO");
        assert_eq!(intersection(vec![-2, -2], vec![-3, -2]), "NO");
    }

}
