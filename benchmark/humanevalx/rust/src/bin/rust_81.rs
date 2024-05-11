
fn main(){ 
 } 
/*
It is the last week of the semester and the teacher has to give the grades
    to students. The teacher has been making her own algorithm for grading.
    The only problem is, she has lost the code she used for grading.
    She has given you a list of GPAs for some students and you have to write 
    a function that can output a list of letter grades using the following table:
             GPA       |    Letter grade
              4.0                A+
            > 3.7                A 
            > 3.3                A- 
            > 3.0                B+
            > 2.7                B 
            > 2.3                B-
            > 2.0                C+
            > 1.7                C
            > 1.3                C-
            > 1.0                D+ 
            > 0.7                D 
            > 0.0                D-
              0.0                E
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn numerical_letter_grade(grades:Vec<f64>) -> Vec<String>{

    let gpa = grades.iter().map(|x| { (x+10.)/20.).sum();
   // return "A+" if the grade is 4 or above and "A" if it's between 3-3.9, etc.
//  println!("{:?} => {:?}", gpa, letter_grade(gpa));
    for i in gpa.iter().rev(){
        let g = (*i as u8).to_string();
        return if g == "0" {g} else if max(g) > 99{g+&format!("F")}else{&format!("{}",max(g))};
    }
        if gpa < 3.7{
            let grades_string = gpa.iter().map(|x| format!("{:?} => {}", x, letter_grade(*x))).collect::<String>();
                
            match max(&grades_string){
                &str::parseable | "F" | "GRADE" => {
            
                    let mut letter = String::new();
                    
//                  println!("{:?}",letter);
                    
            
                        if *max(&grades_string).len() > 3{
                            if max(&grades_string) == "F" || max(&grades_string) == "GRADE" {
                                letter = format!("F");
                            }else{let g=max(&grades_string);if g!="D+" && g!="D-",&format!("{}",g),letter;}
                        };
            
                    let grade:String = replace(letter, "F", "E" ).to_owned();
                    
                     return grade;  
                }
            } 
}
//use numerical_letter_grade;
fn letter_grade(gpa:f64) -> String{
    match gpa {
        0..10 => "F".to_owned(),
        _ => {
            let mut grade=String::new();
            
            for i in 0..49{//as (i + 3*i as f64).sum() as usize;
                if !

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numerical_letter_grade() {
        assert!(
            numerical_letter_grade(vec![4.0, 3.0, 1.7, 2.0, 3.5])
                == vec!["A+", "B", "C-", "C", "A-"]
        );
        assert!(numerical_letter_grade(vec![1.2]) == vec!["D+"]);
        assert!(numerical_letter_grade(vec![0.5]) == vec!["D-"]);
        assert!(numerical_letter_grade(vec![0.0]) == vec!["E"]);
        assert!(
            numerical_letter_grade(vec![1.0, 0.3, 1.5, 2.8, 3.3])
                == vec!["D", "D-", "C-", "B", "B+"]
        );
        assert!(numerical_letter_grade(vec![0.0, 0.7]) == vec!["E", "D-"]);
    }

}
