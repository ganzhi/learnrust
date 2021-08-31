use std::assert;
struct Solution{

}

// Paste below
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        if k <= 0 || s.len() == 0{
            return 0;
        }
        let bytes = s.as_bytes();
        let mut stats = HashMap::<u8, usize>::new();

        let mut pos = 0;
        let mut max_length = 0;
        let mut curr_length = 0;
        let mut start_pos = 0;
        
        loop {
            if pos >= bytes.len() {
                if curr_length > max_length {
                    max_length = curr_length;                    
                }
                break;
            }
            let mut found = false;
            for (j, v) in &mut stats {
                if *j == bytes[pos] {
                    *v = pos;
                    found = true;
                    break;
                }
            }
            if found {
                curr_length += 1;                
                pos += 1;
                continue;
            }

            if (stats.len() as i32) < k {
                stats.insert(bytes[pos], pos);
                curr_length += 1;
                pos += 1;
            } else { // Need to replace one existing character                
                if curr_length > max_length {
                    max_length = curr_length;                    
                }

                let mut to_be_replaced = 255u8;
                let mut pos_min = bytes.len();
                for (j, v) in &stats {
                    if *v < pos_min {
                        pos_min = *v;
                        to_be_replaced = *j;
                    }
                }
                stats.remove(&to_be_replaced);
                stats.insert(bytes[pos], pos);
                println!("Replace {} lastly appeared at {} with {}", to_be_replaced as char, pos_min, bytes[pos] as char);
                println!("Original start pos is {}, now starts from {}", start_pos, pos_min + 1);
                println!("Current lenght is {}", curr_length);
                curr_length = pos - pos_min;
                println!("Current lenght is {}", curr_length);
                start_pos = pos_min + 1;
                pos += 1;
            }
        }
        max_length as i32
    }
}

fn main() {
    let res = Solution::length_of_longest_substring_k_distinct("abdea".to_string(), 2);
    assert!(res == 2, "result is {}", res);
    let res = Solution::length_of_longest_substring_k_distinct("eceba".to_string(), 2);
    assert!(res == 3, "result is {}", res);
    let res = Solution::length_of_longest_substring_k_distinct("aa".to_string(), 2);
    assert!(res == 2, "result is {}", res);
    let res = Solution::length_of_longest_substring_k_distinct("a".to_string(), 0);
    assert!(res == 1, "result is {}", res);    
}
