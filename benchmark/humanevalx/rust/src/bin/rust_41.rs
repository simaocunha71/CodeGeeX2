
fn main(){ 
 } 
/*

    Imagine a road that's a perfectly straight infinitely long line.
    n cars are driving left to right;  simultaneously, a different set of n cars
    are driving right to left.   The two sets of cars start out being very far from
    each other.  All cars move in the same speed.  Two cars are said to collide
    when a car that's moving left to right hits a car that's moving right to left.
    However, the cars are infinitely sturdy and strong; as a result, they continue moving
    in their trajectory as if they did not collide.

    This function outputs the number of such collisions.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn car_race_collision(n:i32)-> i32{

    let n = n.min(10); //cars limited to 10 at most
    let mut hashSet :HashSet<String> = HashSet::new();
    for _ in 0..n {
        hashSet.insert(format!("Car{}", rand_xchg::urandom()));
    }
    println!("{:?}", hashSet);
    let cars :[;N] = (0..N).map(|i|{  hashSet.choose().to_string() }).collect();
    for _ in 0 .. N { //each car is assigned a direction;  the directions can be: left, right, or straight ahead
        let mut direction :Direction = Direction::new();
            match rand_xchg::urandom() % 3{
                0 => direction.set(Direction::Left), //car is driving to the left with speed of cars/N
                1 => direction.set(Direction::Right) ,// 
                2=>direction.set(Direction::Straight){ //no collisions expected if this car is set here
            }
            println!("{:?}", direction);
    }
    let hashSet:Vec<String> = cars .iter().map(|i|{ (i).clone() }).collect();;
    if !hashSet.contains(&"Car0".to_string()) { //car 0 starts at the beginning of the line, and is set to drive straight ahead;
        return 1;};// so it's a collision.
        
    for _ in 0 .. N - 1{
        let mut direction:Direction = Direction::new(); //for each car that is assigned a direction, we have to check if their directions are colliding with another car's, i.e., if cars/i+1 should have started out moving right and cars/i+2 left, then the two cars collide
        let mut collision:bool = false; //this returns true if any of these cars collide on their way down the line
        
        for _ in 0 .. N {
            for j in 0..N - 1 { //for each other pair of cars, we loop over both to see if they collide and return collision
                match(j ,&

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_car_race_collision() {
        assert!(car_race_collision(2) == 4);
        assert!(car_race_collision(3) == 9);
        assert!(car_race_collision(4) == 16);
        assert!(car_race_collision(8) == 64);
        assert!(car_race_collision(10) == 100);
    }

}
