// https://leetcode.com/explore/interview/card/google/61/trees-and-graphs/3068/
struct Solution {

}


// Paste below
use std::collections::HashSet;
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
        for word in word_list.iter() {
            if *word == end_word {
                end_count +=1;
                continue;;
            }
            if *word != begin_word {
                tgt.insert(word.clone());
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
                if Solution::is_adjcent(&s_word, &end_word) {
                    return round;
                }
                for t_word in tgt.iter() {
                    if Solution::is_adjcent(&s_word, t_word) {    
                         println!("{} and {} is adjcent", s_word, t_word);
                        next.insert(t_word.clone());
                    }
                }
            }
            if next.len() == 0 {
                return 0;
            }
             println!("Next is {:?}", next);
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
