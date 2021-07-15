use macro_demo::*;

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
    do_something();
}