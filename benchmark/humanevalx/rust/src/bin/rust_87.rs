
fn main(){ 
 } 
/*

    You are given a 2 dimensional data, as a nested lists,
    which is similar to matrix, however, unlike matrices,
    each row may contain a different number of columns.
    Given lst, and integer x, find integers x in the list,
    and return list of tuples, [(x1, y1), (x2, y2) ...] such that
    each tuple is a coordinate - (row, columns), starting with 0.
    Sort coordinates initially by rows in ascending order.
    Also, sort coordinates of the row by columns in descending order.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn get_row(lst:Vec<Vec<i32>>, x:i32) -> Vec<Vec<i32>>{

    let mut y = 0;
    let mut result:Vec<_> = vec![];
    for col in lst.iter().flatten(){
        if *col == x {
            return vec![col, &y]
        }else{
            y+=1;
        }
    }
    if x != 0 {
        result.push(vec![x])
    } else {
        panic!("Row not found");
    };
    result
}
fn get_col(lst:Vec<i32>, x:i32) -> Vec<i32>{
    for y in 0..len(&lst){
        if *(lst[y]) == x {
            return vec![&x, &y];
        } else {
            let mut idx = len(&lst);
            while idx > -1 && (idx-1) % *(lst[idx-1] as usize) != 0{
                idx -= 1;
            }
            if idx > -1 && x == *((idx-1) % *(lst[idx-1]) as usize + 1){
                
                return vec![x, &idx];
            };
        }
    }
    // panic!("Column not found");
}
impl<T> Iterator for Iters {
    type Item = Vec<(i32, i32)>;
    fn next(&mut self) -> Option<Vec<(i32, i32)>>{
        let mut result:Option<_> = None;
        while let Some(x) = self.iter1.next(){
            if let  Ok(y) = x {
                // println!("X {} y {}", &x,&y);
                if let Some(z) = get_col(&self.lst,*x){
                    result = Some((z,y));
                };
            } else{
                panic!("Invalid input - {}", &x ); 
            };
        };
        if let Some(z) = self.iter2.next(){
            match z {
                Ok(t) => 
                    result = Some((t,y)),
                Err(_) | _ => (),
            }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        assert!(
            get_row(
                vec![
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 1, 6],
                    vec![1, 2, 3, 4, 5, 1]
                ],
                1
            ) == vec![vec![0, 0], vec![1, 0], vec![1, 4], vec![2, 0], vec![2, 5]]
        );
        assert!(
            get_row(
                vec![
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6]
                ],
                2
            ) == vec![
                vec![0, 1],
                vec![1, 1],
                vec![2, 1],
                vec![3, 1],
                vec![4, 1],
                vec![5, 1]
            ]
        );
        assert!(
            get_row(
                vec![
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![1, 1, 3, 4, 5, 6],
                    vec![1, 2, 1, 4, 5, 6],
                    vec![1, 2, 3, 1, 5, 6],
                    vec![1, 2, 3, 4, 1, 6],
                    vec![1, 2, 3, 4, 5, 1]
                ],
                1
            ) == vec![
                vec![0, 0],
                vec![1, 0],
                vec![2, 0],
                vec![2, 1],
                vec![3, 0],
                vec![3, 2],
                vec![4, 0],
                vec![4, 3],
                vec![5, 0],
                vec![5, 4],
                vec![6, 0],
                vec![6, 5]
            ]
        );
        let v: Vec<Vec<i32>> = vec![];
        assert!(get_row(vec![], 1) == v);
        assert!(get_row(vec![vec![1]], 2) == v);
        assert!(get_row(vec![vec![], vec![1], vec![1, 2, 3]], 3) == vec![vec![2, 2]]);
    }

}
