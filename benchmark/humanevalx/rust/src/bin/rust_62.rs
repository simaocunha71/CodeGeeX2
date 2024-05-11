
fn main(){ 
 } 
/*
 xs represent coefficients of a polynomial.
    xs[0] + xs[1] * x + xs[2] * x^2 + ....
     Return derivative of this polynomial in the same form.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn derivative(xs:Vec<i32>) -> Vec<i32>{

    let len = xs.len();
    let mut result = vec![0.; len];
    for idx in 1..len { // get x_idx
        let x_idx = idx as f64;
        
        for (j, (x, xsj)) in xs[..(len - idx)].iter().enumerate()
            .map(|(i, j, x, xsj)| ((*xsj)[idx], i, x * x_idx + (*xsj)[idx-1] as f64, j)).take(5).unzip3()
        {
            
            let delta = (x - 0.0) / (x_idx); // delta is the coefficient of x on derivative[i].
            
            result[j] += delta;
            if f64::abs(&delta) >= f64::EPSILON { // stop when we add a new variable,
                let n = j+1;
                    
                for (k, d) in &xs[n..].iter().take_while(|&(n, _, _, _, _)| max(&delta, d))
            { // replace the last value by the one that's here
                replace(xs.last_mut()?, n);
                result[k-1] -= delta;
            }
        }
    }
    return result;
}
fn main() {
    let mut rnd: Rng = rand::thread_rng(); // rand::random is unsafe, so use rand::ThreadRng.
    let n = 1024; // the number of variables in each derivative.
    let xs = (0..n).map(move |&x| {
        let t: &str;
        
        loop{
            match rnd.gen_u32()%3 {
                0 => return x + 1, // if this is zero, the sequence will be like
                _ => (), // the derivative of f(x) = x^n - c_n (with n = 1)
            };
        }
        1;
    }).collect(); // the resultant is a vector of size n.
    println!("xs[0] + xs[1] *

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derivative() {
        assert!(derivative(vec![3, 1, 2, 4, 5]) == vec![1, 4, 12, 20]);
        assert!(derivative(vec![1, 2, 3]) == vec![2, 6]);
        assert!(derivative(vec![3, 2, 1]) == vec![2, 2]);
        assert!(derivative(vec![3, 2, 1, 0, 4]) == vec![2, 2, 0, 16]);
        assert!(derivative(vec![1]) == vec![]);
    }

}
