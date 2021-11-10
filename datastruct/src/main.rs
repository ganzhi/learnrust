use std::ops::Add;
use std::slice::SliceIndex;
use std::thread;
use std::time::Duration;

use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

fn main() {
    let child1 = Node{value:2, left:None, right: None};
    let child2 = Node{value:1, left:None, right: None};
    let parent = Node {value:0, left:Some(Box::new(child1)), right:Some(Box::new(child2))};
    print!("The value of parent is {}\n", parent.value);
    if let Some(c) = parent.left {
        print!("The value of left child is {}\n", c.value);
    }
    if let Some(c) = parent.right {
        print!("The value of right child is {} \n", c.value);
    }

    let handle = thread::spawn(move || {
        for i in 1..10 {
            print!("The value of parent is {} \n", parent.value);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    let count = Arc::new(AtomicI64::new(0));
    let count_c = count.clone();
    let thread1 = thread::spawn(move|| {
        for i in 0..10000 {
            count_c.as_ref().fetch_add(1, Ordering::SeqCst);
        }
    });
    let count_c = count.clone();
    let thread2 = thread::spawn(move|| {
        for i in 0..10000 {
            count_c.as_ref().fetch_add(1, Ordering::SeqCst);
        }
    });
    let count_c = count.clone();
    let thread3 = thread::spawn(move|| {
        for i in 0..10000 {
            count_c.as_ref().fetch_add(1, Ordering::SeqCst);
        }
    });
    thread1.join();
    thread2.join();
    thread3.join();
    println!("The final count is {:?} \n", count);
}
