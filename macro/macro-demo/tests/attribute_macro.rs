use macro_demo::*;
use std::collections::HashSet;

// macro converts struct S to struct H
#[my_custom_attribute]
struct S{}

#[test]
fn test_macro(){
// due to macro we have struct H in scope
    let demo = H{};
}

#[trace_vars(a)]
fn do_something(){
  let mut a=9;
  a=6;
  a=0;
}

#[test]
fn test_macro1(){
// due to macro we have struct H in scope
  let demo=H{};
  do_something();
}

macro_rules! set {
  ( $($a:expr),*) => {
    {
      let mut s = HashSet::new();
      $(
        s.insert($a);
      )*
      s
    }
  };
}

#[test]
fn test_set_macro(){
  let s = set![1,2,3+8];
  print!("{:?}", s);
}

#[matrix(b)]
fn do_print_token(){
  let mut a=9;
  a=6;
  a=0;
  {
    let a = 6;
    println!("{}", a);
  }
}

#[test]
fn test_print_token(){
  do_print_token();
}