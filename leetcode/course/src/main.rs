use std::assert;
struct Solution {

}

// Paste from below
use std::collections::HashMap;
use std::collections::HashSet;
use std::cell::RefCell;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        if num_courses == 0 {
            return true;
        }
        let mut graph = HashMap::<i32, RefCell<HashSet<i32>>>::with_capacity(prerequisites.len());
        for item in prerequisites.iter() {
            match graph.get_mut(&item[0]) {
                Some(s) => {
                    s.borrow_mut().insert(item[1]);
                },
                None => {
                    let mut news = HashSet::<i32>::new();
                    news.insert(item[1]);
                    graph.insert(item[0], RefCell::new(news));
                },
            }
        }

        loop {
            let mut modifed = false;
            for value in graph.values(){
                let mut v = value.borrow_mut();
                let len = v.len();
                v.retain(|&k| graph.contains_key(&k));
                if v.len() != len {
                    modifed = true;
                }
            }
            graph.retain(|_, v| !v.borrow().is_empty());

            if graph.is_empty() {
                return true;
            }

            if !modifed {
                return false;
            }
        }
    }
}

fn main() {
    assert!(Solution::can_finish(2, vec![vec![1, 0]]));
    assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
}
