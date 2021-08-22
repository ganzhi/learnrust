// https://leetcode.com/explore/interview/card/google/61/trees-and-graphs/3068/
struct Solution {

}


// Paste below
use std::collections::HashSet;
use std::collections::HashMap;
impl Solution {
    fn is_adjcent(s1: &String, s2: &String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        if s1.len() != s2.len() {
            return false;
        }
        let mut distance = 0;
        for i in 0..s1.len()  {
            if s1[i] != s2[i] {
                distance += 1;
            }
            if distance>1 {
                return false;
            }
        }
        return true;
    }

    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut src = HashSet::new();
        src.insert(begin_word.clone());
        let mut tgt = HashSet::new();
        let mut end_count = 0;
        let mut graph = HashMap::<String, HashSet<String>>::new(); 
        for word in word_list.iter() {            
            if *word == begin_word {
                continue;
            }
            if *word == end_word {
                end_count +=1;
            }
            tgt.insert(word.clone());
            
            for i in 0..word.len() {
                let mut cloned_bytes = word.clone().into_bytes();
                cloned_bytes[i] = b'*';
                let root = String::from_utf8(cloned_bytes).unwrap();
                match graph.get_mut(&root) {
                    Some(v) =>{
                        v.insert(word.clone());
                    },
                    None => {
                        let mut same_root = HashSet::new();
                        same_root.insert(word.clone());
                        graph.insert(root, same_root);
                    },
                }
                
            }
        }
        if end_count == 0 {
            return 0;
        }

        let mut round = 1;
        loop {
            round += 1;
            let mut next = HashSet::new();
            for s_word in src.iter(){
                for i in 0..s_word.len() {
                    let mut cloned_bytes = s_word.clone().into_bytes();
                    cloned_bytes[i] = b'*';
                    let root = String::from_utf8(cloned_bytes).unwrap();
                    match graph.get_mut(&root) {
                        Some(v) =>{
                            for neighbour in v.iter(){
                                if tgt.contains(neighbour) {
                                    if *neighbour == end_word {
                                        return round;
                                    }
                                    tgt.remove(neighbour);
                                    next.insert(neighbour.clone());
                                }
                            }
                        },
                        None => {},
                    }
                    
                }
            }
            //println!("Next is {:?}, target is {:?}", next, tgt);
            if next.len() == 0 {
                return 0;
            }
            
            for n_word in next.iter() {
                tgt.remove(n_word);
            }
            src = next;
        }      
    }
}

fn main() {
    let dict = vec!["hot","dot","dog","lot","log","cog"];
    let dict1 = dict.iter().map(|f| f.to_string()).collect();
    let res = Solution::ladder_length("hit".to_string(), "cog".to_string(), dict1);
    println!("The result is {}", res);
}
