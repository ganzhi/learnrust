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

        let mut checked = HashSet::<i32>::new();

        for (k, v) in graph.iter(){
            if checked.contains(k) {
                continue;
            }
            let mut visited = HashSet::<i32>::new();
            let mut queue = Vec::<i32>::new();
            queue.push(*k);
            while !queue.is_empty() {
                let next = queue.pop().unwrap();
                checked.insert(next);
                if visited.contains(&next) {
                    return false;
                }
                visited.insert(next);
                if let Some(neighbours) = graph.get(&next) {
                    for i in neighbours.borrow().iter() {
                        if visited.contains(i) {
                            return false;
                        }
                        queue.push(*i);
                    }
                }
            }
        }
        true
    }
}

fn main() {
    // assert!(Solution::can_finish(2, vec![vec![1, 0]]));
    // assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    assert!(Solution::can_finish(2, vec![vec![1,4], vec![2,4], vec![3,1], vec![3,2]]));    
}
