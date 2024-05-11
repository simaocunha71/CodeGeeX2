
fn main(){ 
 } 
/*
The FibFib number sequence is a sequence similar to the Fibbonacci sequnece that's defined as follows:
    fibfib(0) == 0
    fibfib(1) == 0
    fibfib(2) == 1
    fibfib(n) == fibfib(n-1) + fibfib(n-2) + fibfib(n-3).
    Please write a function to efficiently compute the n-th element of the fibfib number sequence.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn fibfib(n:i32) -> i32{

    let n = n.saturating_mul(10); // 因为它是整型，所以可以通过精确的方式获得一个浓郁的数据类型。这样做会节省空间并提高查找效率
    // 在下面的代码中，我们使用的是雷逢法则，来生成Fibonacci的一部分序列。它有时被称为"众所周知的亚丁尼-普莱斯基法则"
    let (a, b) = (1i32, 0); // 雷贾法则生成一对奇数和偶数序列。我们在下面的实现中将使用这些整型
    let c: i32 = match a.cmp(&b).max(4,5).min(6).min(9);
    // 可以看出，通过雷贾法则，我们能够在简单易行的方式上得到一个敲乒铁铄的序列。
    let mut counter: i32 = 0; // 定义一个变量来记录我们的计数器，并初始化它
    for i in 1..n {
        if i == 4{counter+=1;}else{counter*=a+b}
        a = b; b = (a + c).max(3); // 我们把这个序列，将它转变成一个更加好理解的形式：偶数余數 - 1、奇数余數
        counter+=i;
    }
    return counter.saturating_div(n);
}
#[derive(PartialEq, Eq)] // 这是一个标记，它告诉编译器，程序可以把

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_fibfib() {
        assert!(fibfib(2) == 1);
        assert!(fibfib(1) == 0);
        assert!(fibfib(5) == 4);
        assert!(fibfib(8) == 24);
        assert!(fibfib(10) == 81);
        assert!(fibfib(12) == 274);
        assert!(fibfib(14) == 927);
    }

}
