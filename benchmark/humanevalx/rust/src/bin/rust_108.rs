
fn main(){ 
 } 
/*

    Write a function count_nums which takes an array of integers and returns
    the number of elements which has a sum of digits > 0.
    If a number is negative, then its first signed digit will be negative:
    e.g. -123 has signed digits -1, 2, and 3.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn count_nums(n:Vec<i32>) -> i32{

    let mut count = 0;
    for (x,y) in &mut n.iter() {
        if *x > -10 && *x < 10 {
            if *y == 0 || (*y % 2) != 0 {
                replace(&n, index_mut(*x as usize), y);
                count += 1;
            } else {
                let mut v = n.clone();
                match *(v.iter_mut().next()).unwrap() {
                    i if TypeId::of::<u32>().is_assignable_from(&TypeId::of::<u8>()) => {
                        replace(&n, index_mut(*x as usize), i);
                        count += 1;
                    }
                    _ =>{
                        let mut set = HashSet::new();
                        for (i,y) in n.iter_mut().enumerate() {
                            if replace(&n, index_mut(*i as usize), y).abs() < 10 {
                                *replace(&set,&type_id!("u8"))=0;
                            } else{
                                let v:Any = n.clone();
                                v.into_iter().for_each(|(i,y)| {
                                    let (num,digit) = i.cmp(&TypeId::of::<(u8, u16, u32)>());
                                    if digit == 0 || (*v[&num]).abs() % 2 != 0 {
                                        *replace(&set,&type_id!("u8"))=0;
                                    }
                                });
                            }
                        }
                    set.retain(|x| x > 0 && replace(&n, index_mut(*i as usize), &x).abs() < 10);
                        if let Ok(v) = v {
                            *replace(&n,index_mut(*x as usize),&*v)!=0;
                        } else {
                            *replace(&n,index_mut(*x as usize),&"u8")=0;
                        }
                    }
                };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_nums() {
        assert!(count_nums(vec![]) == 0);
        assert!(count_nums(vec![-1, -2, 0]) == 0);
        assert!(count_nums(vec![1, 1, 2, -2, 3, 4, 5]) == 6);
        assert!(count_nums(vec![1, 6, 9, -6, 0, 1, 5]) == 5);
        assert!(count_nums(vec![1, 100, 98, -7, 1, -1]) == 4);
        assert!(count_nums(vec![12, 23, 34, -45, -56, 0]) == 5);
        assert!(count_nums(vec![-0, 1]) == 1);
        assert!(count_nums(vec![1]) == 1);
    }

}
