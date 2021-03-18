trait MyIter {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct MyIterator<'a, T:Clone> {
    index:usize,
    vv: &'a mut Vec<T>
}

impl<'a, T:Clone> MyIterator<'a, T> {
    fn new(v: &'a mut Vec<T>) -> Self {
        MyIterator{index:0, vv:v}
    }
}

impl<'a, T:Clone> MyIter for MyIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T>
    {
        let ind = self.index;
        if ind < self.vv.len() {
            self.index = ind + 1;
            return Some(self.vv[ind].clone());
        }
        return None;
    }
}

struct MyVec<T> {
    v: Vec<T>,
}

impl<T:Clone> MyVec<T> {
    fn new() -> Self {        
        MyVec{
            v: Vec::new()
        }
    }

    fn iter<'a>(self: &'a mut Self) -> MyIterator<'a, T>{
        MyIterator::new(&mut self.v)
    }
}

use std::ops::Deref;
use std::ops::DerefMut;

impl<'a, T:Clone> Deref for MyIterator<'a, T>{
    type Target = T;

    fn deref(&self) -> &T {
        &self.vv[self.index]
    }
}

impl<'a, T:Clone> DerefMut for MyIterator<'a, T>{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.vv[self.index]
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let mut myvect = MyVec::new();
    for i in 1..10 {
        myvect.v.push(i);
    }

    let mut iter = myvect.iter();
    println!("Original head element is {}", *iter);
    
    *iter = 99;
    while let Some(v) = iter.next() {
        println!("{}", v);
    }    
}