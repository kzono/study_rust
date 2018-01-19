
pub fn sum_n(n: i32) -> i32{
  if n == 1 { 1 }
  else { n + sum_n(n-1) }
}

pub fn fact(n: i32) -> i32{
  if n == 1 { 1 }
  else { n * fact(n-1) }
}

pub fn fib(n: i32) -> i32{
  if n == 0 { 0 }
  else if n == 1 { 1 }
  else  { fib(n-2) + fib(n-1) }
}


