struct Solution {

}

use std::cmp;
use std::collections::HashMap;

impl Solution {
    fn min_dist_internal(word1:&[u8], pos1:usize, word2: &[u8], pos2:usize, buf: &mut HashMap::<Vec<usize>, usize>) -> usize {
        let key = vec![pos1, pos2];
        if let Some(v) = buf.get(&key) {
            return *v;
        }

        if pos1 == word1.len() {
            return word2.len() - pos2;
        }

        if pos2 == word2.len() {
            return word1.len() - pos1;
        }
        
        if word1[pos1] == word2[pos2]{
            return Solution::min_dist_internal(word1, pos1+1, word2, pos2+1, buf);
        }

        let r1 = Solution::min_dist_internal(word1, pos1, word2, pos2+1, buf);
        let r2 = Solution::min_dist_internal(word1, pos1+1, word2, pos2, buf);
        let dist = cmp::min(r1, r2) + 1;

        buf.insert(key, dist);
        return dist;
    }

    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut buf = HashMap::<Vec<usize>, usize>::new();
        Solution::min_dist_internal(word1.as_bytes(), 0, word2.as_bytes(), 0, &mut buf) as i32
    }
}

fn main() {
    let word1 = String::from("leetcode");
    let word2 = String::from("etco");
    let distance = Solution::min_distance(word1, word2);
    println!("Delete distance {}", distance);
}
