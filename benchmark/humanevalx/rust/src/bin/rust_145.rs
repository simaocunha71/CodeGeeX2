
fn main(){ 
 } 
/*

    Write a function which sorts the given list of integers
    in ascending order according to the sum of their digits.
    Note: if there are several items with similar sum of their digits,
    order them based on their index in original list.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn order_by_points(arr: Vec<i32>) -> Vec<i32> {

    let (_, points) = arr.iter().enumerate();
    order_by_points_with_predecessors(&arr, points).to_vec()
}
fn order_by_points_with_predecessors(arr: Vec<i32>, points: Vec<usize>) -> Vec<i32> {
    let mut prev_index = None;
    let points_hash_map = HashMap::new();
    for (idx, point) in &points[..] {
        if !points_hash_map.contains_key(&idx) {
            points_hash_map.insert(prev_index = Some(idx), *point);
        } else {
            let (prev_i32, p1) = points_hash_map.iter().position(|&(prev_idx, _)| prev_idx == &prev_index).unwrap();
            if let Ok((prev_prev_i32, _) = arr.get_unchecked(&p1) {
                *point - replace(prev_prev_i32, 0).min(prev_prev_i32).max(-1) as i32 + prev_i32
            } else {
                0
            });
        }
    }
    let mut it = points_hash_map.into_iter();
    it.collect::<Vec<(_, i32, _)>>().to_vec()
}
fn order_by_points(arr: Vec<i32>) -> Vec<usize> {
    let mut prev_index = None;
    let points_hash_map = if arr.len() % 4 != 0 {
        panic!();
    } else {
        let points = arr.iter().enumerate();
        let points_hash_map = HashMap::new();
        for (idx, point) in &points[..] {
            if !points_hash_map.contains_key(&idx) {
                points_hash_map.insert(prev_index = Some(idx), *point);
            } else {
                let (prev_i32, p1) = points_hash_map.iter().position(|&(prev_

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_by_points() {
        assert_eq!(
            order_by_points(vec![1, 11, -1, -11, -12]),
            vec![-1, -11, 1, -12, 11]
        );
        assert_eq!(
            order_by_points(vec![
                1234, 423, 463, 145, 2, 423, 423, 53, 6, 37, 3457, 3, 56, 0, 46
            ]),
            vec![0, 2, 3, 6, 53, 423, 423, 423, 1234, 145, 37, 46, 56, 463, 3457]
        );
        assert_eq!(order_by_points(vec![]), vec![]);
        assert_eq!(
            order_by_points(vec![1, -11, -32, 43, 54, -98, 2, -3]),
            vec![-3, -32, -98, -11, 1, 2, 43, 54]
        );
        assert_eq!(
            order_by_points(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
            vec![1, 10, 2, 11, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(
            order_by_points(vec![0, 6, 6, -76, -21, 23, 4]),
            vec![-76, -21, 0, 4, 23, 6, 6]
        );
    }

}
