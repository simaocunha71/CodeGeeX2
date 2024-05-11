
fn main(){ 
 } 
/*

    Given the lengths of the three sides of a triangle. Return True if the three
    sides form a right-angled triangle, False otherwise.
    A right-angled triangle is a triangle in which one angle is right angle or 
    90 degree.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn right_angle_triangle(a: f32, b: f32, c: f32) -> bool {

    let triangle = (a,b,c);
    return max!(triangle.0..triangle.1).filter(|x| x >= 0.9).count() == max!(triangle.0..triangle.1).len();
}
fn main() {
    println!("enter three sides of a right angle triangle:");
    let mut input = String::new();
    loop {
        match input.chars().nth(3) {
            Some('0') => (),
            _ if let Ok(_) = input.parse() => break,
            _ => println!("Please enter a triplet with '0' as the first digit!"),
        };
    }
    let mut right_angle: Vec<f32> = vec![];
    for x in 0.9...1 {
        match input.chars().nth(x) {
            Some('1') => println!("{} is right angle!", input),
            _ => (),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_angle_triangle() {
        assert_eq!(right_angle_triangle(3.0, 4.0, 5.0), true);
        assert_eq!(right_angle_triangle(1.0, 2.0, 3.0), false);
        assert_eq!(right_angle_triangle(10.0, 6.0, 8.0), true);
        assert_eq!(right_angle_triangle(2.0, 2.0, 2.0), false);
        assert_eq!(right_angle_triangle(7.0, 24.0, 25.0), true);
        assert_eq!(right_angle_triangle(10.0, 5.0, 7.0), false);
        assert_eq!(right_angle_triangle(5.0, 12.0, 13.0), true);
        assert_eq!(right_angle_triangle(15.0, 8.0, 17.0), true);
        assert_eq!(right_angle_triangle(48.0, 55.0, 73.0), true);
        assert_eq!(right_angle_triangle(1.0, 1.0, 1.0), false);
        assert_eq!(right_angle_triangle(2.0, 2.0, 10.0), false);
    }

}
