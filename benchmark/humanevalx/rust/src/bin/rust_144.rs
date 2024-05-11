
fn main(){ 
 } 
/*
Your task is to implement a function that will simplify the expression
    x * n. The function returns True if x * n evaluates to a whole number and False
    otherwise. Both x and n, are string representation of a fraction, and have the following format,
    <numerator>/<denominator> where both numerator and denominator are positive whole numbers.

    You can assume that x, and n are valid fractions, and do not have zero as denominator.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn simplify(x: &str, n: &str) -> bool {

    let (n_num, n_denom): (i16, i64) = (&n[..], "0".parse().unwrap());
    if max(&n_num, &x[..]) > x.len() as i64 || !(0 <= x.chars().count()) {
        println!("{}: {}", n, x);
        return false;
    }
    let x = x.parse::<f64>().unwrap();
    match n_denom.cmp(&x) {
        cmp::Ordering::Less => false,
        cmp::Ordering::Equal => println!("{}: {}", n, x),
        _ => true,
    }
}
fn test() -> () {
    let mut n: i64 = 0;
    for (x, n) in INPUT_TEST.iter().cloned() {
        if simplify(x, &format!("{}", n)) {
            println!("{}: {}", x, n);
        }
    }
}
#[cfg(test)]
mod tests {
    use test_util::*;
    #[derive(Default)]
    struct NTestCase {
        x: f64
    };
    impl<'x> Test for NTestCase<'x> {
        fn set_up<T, U>(&self, t: &mut T, u: &U) where U : FnMut(&NTestCase<'x>) -> () {}
        fn get_result<R>(&self) -> R where R : Clone + Default + Debug + Eq + PartialEq + Hash + Copy {
            self.x.clone().unwrap()
        }
    }
    #[test]
    fn it_should_be_true_when_n() {
        let test = NTestCase { x: 15f64 };
        assert_eq!(simplify("20", &format!("{}", test.get_result())), true);
        assert_eq!(simplify("33f64", "10f64"), true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify() {
        assert_eq!(simplify("1/5", "5/1"), true);
        assert_eq!(simplify("1/6", "2/1"), false);
        assert_eq!(simplify("5/1", "3/1"), true);
        assert_eq!(simplify("7/10", "10/2"), false);
        assert_eq!(simplify("2/10", "50/10"), true);
        assert_eq!(simplify("7/2", "4/2"), true);
        assert_eq!(simplify("11/6", "6/1"), true);
        assert_eq!(simplify("2/3", "5/2"), false);
        assert_eq!(simplify("5/2", "3/5"), false);
        assert_eq!(simplify("2/4", "8/4"), true);
        assert_eq!(simplify("2/4", "4/2"), true);
        assert_eq!(simplify("1/5", "5/1"), true);
        assert_eq!(simplify("1/5", "1/5"), false);
    }

}
