struct Solution {

}


impl Solution {
    fn is_palindrome(num:u64) -> bool {
        let s = num.to_string();
        let b = s.as_bytes();
        let mut l: usize = 0;
        let mut r = b.len() - 1;
        loop {
            if l >= r {
                return true;
            }
            if b[l] == b[r] {
                l += 1;
                r -= 1;
            } else {
                return false;
            }
        }
    }

    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let left = left.parse::<u64>().unwrap();
        let right = right.parse::<u64>().unwrap();

        let mut sleft = (left as f32).sqrt() as u64;
        if sleft * sleft > left {
            sleft += 1;
        }
        let sright = (right as f32).sqrt() as u64;

        let min_len = sleft.to_string().len();
        let max_len = sright.to_string().len();

        let mut count = 0;

        for pal_len in min_len..(max_len+1){
            let half = pal_len/2;
            let upper = (10 as u64).pow(half as u32);
            for i in 0..upper {
                let mut rev = i.to_string();
                while rev.len()<half{
                    rev.insert(0, '0');
                }
                let rev:String = rev.chars().rev().collect();
                let rev = rev.parse::<u64>().unwrap();
                if pal_len %2 == 0{
                    let combined = rev * upper + i;
                    if combined >= sleft && combined <= sright {
                        let sq = combined * combined;
                        if Solution::is_palindrome(sq) {
                            count += 1;
                        }
                    }
                } else {
                    let comb = rev * upper * 10 + i;
                    for m in 0..10 {
                        let combined = comb + m * upper;
                        if combined >= sleft && combined <= sright {
                            let sq = combined * combined;
                            if Solution::is_palindrome(sq) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }

        count
    }
}

fn main() {
    let res = Solution::superpalindromes_in_range(String::from("4"), String::from("1000"));
    println!("Hello, world: {}", res);

    let res = Solution::superpalindromes_in_range(String::from("462"), String::from("154370712829064"));
    println!("Hello, world: {}", res);

}

/*
Let's say a positive integer is a super-palindrome if it is a palindrome, and it is also the square of a palindrome.

Given two positive integers left and right represented as strings, return the number of super-palindromes integers in the inclusive range [left, right].

 

Example 1:

Input: left = "4", right = "1000"
Output: 4
Explanation: 4, 9, 121, and 484 are superpalindromes.
Note that 676 is not a superpalindrome: 26 * 26 = 676, but 26 is not a palindrome.

Example 2:

Input: left = "1", right = "2"
Output: 1
*/