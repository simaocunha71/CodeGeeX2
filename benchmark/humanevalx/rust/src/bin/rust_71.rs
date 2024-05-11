
fn main(){ 
 } 
/*

    Given the lengths of the three sides of a triangle. Return the area of
    the triangle rounded to 2 decimal points if the three sides form a valid triangle. 
    Otherwise return -1
    Three sides make a valid triangle when the sum of any two sides is greater 
    than the third side.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn triangle_area_f64(a:f64, b:f64, c:f64) -> f64{

    let r1 = a.recip();
    let r2 = (b + c).recip();
    //let r3 = match a < r1 {false => 0., true => r1}
    //let r4 = match b < r2 {false => 0., true => r2}
    //let r5 = match c < r3 {false => 0., true => r3}
    let r6 = (r1 + r2).recip();
    let r7 = ((b * c) as f64 - a * r6);
    let r8 = (((a * b) as f64) - (c * r7));
    match r8.fma(r7, r6) {
        Ok(res) => res,
        Err(_)=>0.
    }
}
#[derive(Debug)]
struct ValidTri {
    v1:i32,
    v2:i32,
    v3:i32
};
#[derive(Clone, Debug)]
enum Invariant { UniquelyPositive, NoTriangles, None,  Triangle };
enum Verdict { Valid, Falsy, None };
enum TriangulatorResult {
    /// A triangle with given sides exists.
    True(ValidTri, &'a [u8], u64),
    /// Given data is not a valid triangle.
    False(&'a [u8]),
    /// Given data is invalid. 
    Error(Error)
}
/// A type to represent the invariants of some type of data.
#[derive(Debug, PartialEq)]
enum Invariant<T> {
    UniquelyPositive(T),
    NoTriangles(),
    None()
};
impl<T:TypeId+PartialEq>PartialEq for Invariant<T>{
    #[inline] fn eq(&self, other:&Self) -> bool{ 
        match self {
            Self::UniquelyPositive(ref a) => a == &other,
            Self::NoTriangles() => true,
            Self::None()=>false, }
    }
}
impl<T:TypeId

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_area_f64() {
        assert!(f64::abs(triangle_area_f64(3.0, 4.0, 5.0) - 6.00) < 0.01);
        assert!(f64::abs(triangle_area_f64(1.0, 2.0, 10.0) + 1.0) < 0.01);
        assert!(f64::abs(triangle_area_f64(4.0, 8.0, 5.0) - 8.18) < 0.01);
        assert!(f64::abs(triangle_area_f64(2.0, 2.0, 2.0) - 1.73) < 0.01);
        assert!(f64::abs(triangle_area_f64(1.0, 2.0, 3.0) + 1.0) < 0.01);
        assert!(f64::abs(triangle_area_f64(10.0, 5.0, 7.0) - 16.25) < 0.01);
        assert!(f64::abs(triangle_area_f64(2.0, 6.0, 3.0) + 1.0) < 0.01);
        assert!(f64::abs(triangle_area_f64(1.0, 1.0, 1.0) - 0.43) < 0.01);
        assert!(f64::abs(triangle_area_f64(2.0, 2.0, 10.0) + 1.0) < 0.01);
    }

}
