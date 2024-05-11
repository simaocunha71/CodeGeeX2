
fn main(){ 
 } 
/*

    You are given a positive integer n. You have to create an integer array a of length n.
        For each i (1 ≤ i ≤ n), the value of a[i] = i * i - i + 1.
        Return the number of triples (a[i], a[j], a[k]) of a where i < j < k, 
    and a[i] + a[j] + a[k] is a multiple of 3.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn get_matrix_triples(n: i32) -> i32 {

    let (mut a, b): (i32[], i64);
    let mut triples: HashSet<(i32[], i32[], i32)> = HashSet::new();
    let n = n as i32;
    let mut counter = 1;
    a = (0..n).take_while(|&x| x != 0).collect::<Vec<i32>>().into_iter().rev();
    b = &a[..].to_vec();
    replace(&mut a, n as i32, counter);
    triples.insert((a, b));
    let mut t: Vec<(i32[], i32[], i32)> = vec![(0, 1, max(triples.len(), 1))];
    for (j,k) in triples {
        if j != 0 && k > 0{
            t.push((j-1, k - 1,j * k + j));
        }else if k < j{
            t.push_back((j-1 , k+1, (k-1)*(j-1)));    
        }
    }
    triples = t;
    assert!(triples.len() == 3);    
    let mut counter = 1 + max(0, triples.len());
    for i in 1..n {
        if a[i] % 3 != 0{
            counter += triples.remove(&(a[i], b[i]))?;
            replace(&mut &mut a[i-1], i as i32, (counter - 1));    
        }else{
            counter -= triples.remove(&(a[i], b[i]))?;        
            if *counter < i {
                assert_eq!((&a[*counter] as u8).to_string(), &format!("{}", a[*counter]));
                replace(&mut &mut a[*counter], counter, i as i32);    
            }
        }
    }
    *counter += triples.remove(&(a[0], b[n]))?;        
    assert_eq!((&a

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_matrix_triples() {
        assert_eq!(get_matrix_triples(5), 1);
        assert_eq!(get_matrix_triples(6), 4);
        assert_eq!(get_matrix_triples(10), 36);
        assert_eq!(get_matrix_triples(100), 53361);
    }

}
