trait MyIter {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct MyIterator<'a, T:Clone> {
    index:usize,
    vv: &'a Vec<T>
}

impl<'a, T:Clone> MyIterator<'a, T> {
    fn new(v: &'a Vec<T>) -> Self {
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

    fn iter<'a>(self: &'a Self) -> MyIterator<'a, T>{
        MyIterator::new(&self.v)
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
    while let Some(v) = iter.next() {
        println!("{}", v);
    }
}