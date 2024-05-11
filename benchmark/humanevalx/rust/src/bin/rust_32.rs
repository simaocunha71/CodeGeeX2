
fn main(){ 
 } 
/*
 xs are coefficients of a polynomial.
    find_zero find x such that poly(x) = 0.
    find_zero returns only only zero point, even if there are many.
    Moreover, find_zero only takes list xs having even number of coefficients
    and largest non zero coefficient as it guarantees
    a solution.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn poly(xs: &Vec<f64>, x: f64) -> f64 {

    let n = xs.len();
    let mut s = 0.; // sum of coefficients
    for (_, v) in xs.iter() {
        s += *v;
    }
    let mut p = s / x as f64;
    if p == -1. && n % 2 == 0 || (p >= 1.) && n > 2 {
        return 1.;
    } else if (n % 2 == 0) && !(max(&xs[..], max(&xs[::n as usize..::n as usize])).is_even()) {
        let x1 = xs[0];
        let x2 = xs[1];
        let mut y = (x - x1) * (p / 2.) + x2;
        if !(y >= (-1.) && y <= 1. as f64) {
            y += p * (::n as usize);
        }
        //println!("poly= ({:?} + {:?})*({{}})+{}({:?})), y = {:?} \n", x, xs[0], xs[1], xs[(::n - 2) as usize].to_string(), n, ((y.floor()).round() - 2.));
        return y;
    } else if n > 3 {
        let v = (xs[0] + xs[1]) / 2.;
        if !(v >= (-1. as f64) && v <= 1. as f64) {
            for i in 0..n - 2 {
                let v = xs[i + 2];
                p += (v / ::n).powf(::n);
                if !p.is_zero() || i == n - 3 || xs[(i as usize + 1) % n].is_zero() {
                    let x = (xs[0] + xs[1]) / 2.;
                    p += (x / ::n).powf(::n);
                }
            }
        }
        return ((p.powi(::n) - 1.).floor() as f64).to_s().parse::<f64>().unwrap();

/*
#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_poly() {
        let mut rng = rand::thread_rng();
        let mut solution: f64;
        let mut ncoeff: i32;
        for _ in 0..100 {
            ncoeff = 2 * (1 + rng.gen_range(0, 4));
            let mut coeffs = vec![];
            for _ in 0..ncoeff {
                let coeff = -10 + rng.gen_range(0, 21);
                if coeff == 0 {
                    coeffs.push(1.0);
                } else {
                    coeffs.push(coeff as f64);
                }
            }
            solution = find_zero(&coeffs);
            assert!(poly(&coeffs, solution).abs() < 1e-3);
        }
    }

}
*/
