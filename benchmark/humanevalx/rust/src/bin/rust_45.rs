
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

fn triangle_area(a:i32, h:i32) -> f64{

    let area = (h - a)*(h - a);  
    if area == 0.f64 {
        return Err("AH".to_string());
    }else{
        area
    }
}
#[derive(PartialEq, Eq, Debug)]
struct Triple(i32, i32, f64);
impl Any for Triple {
    fn as_any(&self) -> &dyn Any + 'static 
    where Self: Sized 
    {
        self.as_any()
    }
}
impl TypeId<Triple> for () {}
impl FromStr for Triple{
    type Err = ();
    fn from_str(s:&str) -> Result<Self, Self::Err>{
        let (a, b, c):(i32, i32, f64 ) =
            s.split_once(',').map(|x| x.parse().map_or(&(0 as f64), |s:&str| Ok((s).parse())));
        if a>=b && (b>=a || c) { 
           let d=(a-b)+((c-b)*a);
            return Ok(Triple(a, b, d)); }else{
                Err("invalid input".to_string())
    }
}
struct Triple2Struct(i32, i32, f64);
impl PartialEq for Triple2Struct {
    fn eq(&self, other: &Self) -> bool 
    where Self: Sized + PartialEq 
    {
        self.0 == other.0 && self.1 == other.1 && (other as &Triple).2 == (self.2 as &f64)
    }    
}
impl Eq for Triple2Struct {}
#[derive(Clone, Debug)]
struct Triangle(i32, i32, f64);
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool 
    where Self: Sized + PartialEq 
    {
        self.0 == other.0 && self.1 == other.1 && (other as

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_area() {
        assert!(triangle_area(5, 3) == 7.5);
        assert!(triangle_area(2, 2) == 2.0);
        assert!(triangle_area(10, 8) == 40.0);
    }

}
