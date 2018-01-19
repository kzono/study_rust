extern crate sums;

use sums::*;


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn sum_n_test(){
    assert_eq!(1, sum_n(1));
    assert_eq!(3, sum_n(2));
    assert_eq!(6, sum_n(3));
    assert_eq!(10, sum_n(4));
  }
  #[test]
  fn fact_test(){
    assert_eq!(1, fact(1));
    assert_eq!(2, fact(2));
    assert_eq!(6, fact(3));
    assert_eq!(24, fact(4));
  }
  #[test]
  fn fib_test(){
    assert_eq!(0, fib(0));
    assert_eq!(1, fib(1));
    assert_eq!(1, fib(2));
    assert_eq!(2, fib(3));
    assert_eq!(3, fib(4));
    assert_eq!(5, fib(5));
  }
}
