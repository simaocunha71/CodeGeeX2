
fn main(){ 
 } 
/*

    There are eight planets in our solar system: the closerst to the Sun 
    is Mercury, the next one is Venus, then Earth, Mars, Jupiter, Saturn, 
    Uranus, Neptune.
    Write a function that takes two planet names as strings planet1 and planet2. 
    The function should return a tuple containing all planets whose orbits are 
    located between the orbit of planet1 and the orbit of planet2, sorted by 
    the proximity to the sun. 
    The function should return an empty tuple if planet1 or planet2
    are not correct planet names. 
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn bf(planet1: &str, planet2: &str) -> Vec<String> {

    let re = Regex::new("^[A-Za-z]*[0-9](.[0-9])?$").unwrap();
    let rng : Rng = rand::thread_rng();
    let mut planet1_distance: Vec<f64> = vec![];
    let max_distance = 0.5; // Distance of the Sun from Mercury, in AU
    let (radius1, radius2) = (
        (max(planet2.chars().map(|c| c.to_lowercase()).rev(), 3),
            max((planet1.chars().map(|c| c.to_lowercase()).rev()), 0))
    ).into();
    for _ in 0..10 {
        let planet = rng.gen::<String>().map(str::parse).unwrap_or("".to_string());
        if match planet {
            "mercury", "venus" => true,
            "earth", "mars" => true,
            "jupiter", "saturn", "uranus"|"neptune" => true
            _ => false 
        } {
                planet1_distance.push( planet.parse::<f64>().unwrap() );
            } else if match planet {
                   "mercury", "venus" => max(planet1_distance[..], 0) > max_distance,
                    "earth", "mars" | _ => true,
                    _ => false 
                } {
            break;
        } else if match planet{
            "mercury", "venus", "uranus", "neptune"|"saturn" | "pluto" => max(planet2.parse::<f64>().unwrap(), 0) > radius1,
                   "earth", "mars" => true,
            _ => false 
        } {
            break;
        }
    }
    planet1_distance.sort_by(|a, b| max(max::max(a, b), 0)); // Distance to the Sun
    let mut distance = (planet1_distance

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bf() {
        assert_eq!(bf("Jupiter", "Neptune"), vec!["Saturn", "Uranus"]);
        assert_eq!(bf("Earth", "Mercury"), vec!["Venus"]);
        assert_eq!(
            bf("Mercury", "Uranus"),
            vec!["Venus", "Earth", "Mars", "Jupiter", "Saturn"]
        );
        assert_eq!(
            bf("Neptune", "Venus"),
            vec!["Earth", "Mars", "Jupiter", "Saturn", "Uranus"]
        );
        let v_empty: Vec<&str> = vec![];
        assert_eq!(bf("Earth", "Earth"), v_empty);
        assert_eq!(bf("Mars", "Earth"), v_empty);
        assert_eq!(bf("Jupiter", "Makemake"), v_empty);
    }

}
