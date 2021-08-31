struct Solution {

}

impl Solution {
    fn validate(buf: &mut Vec<i32>, pos:usize) -> bool {
        for i in 0..pos{
            if buf[i] == buf[pos] {
                return false;
            }
        }

        for i in 0..pos{
            if (pos-i) as i32 == (buf[pos]-buf[i]) {
                return false
            }
            if (pos-i) as i32 == (buf[i]-buf[pos]) {
                return false
            }            
        }
        true
    }

    fn update_pos(buf: &mut Vec<i32>, pos:usize, n:usize) -> usize{
        let ni = n as i32;
        for v in (buf[pos]+1)..ni  {
            buf[pos] = v;
            if Solution::validate(buf, pos) {
                return pos;
            }
        }
        buf[pos] = -1;
        pos - 1
    }

    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut buf = vec![-1; n];
        let mut sum = 0;
        let mut pos = 0 as usize;
        loop {
            let next_pos = Solution::update_pos(&mut buf, pos, n);
            if next_pos == pos {
                if pos == n -1 {
                    sum = sum + 1;
                } else {
                    pos = pos +1;
                }
            } else {
                pos = next_pos;
                for i in pos+1..n {
                    buf[i as usize] = -1;
                }
            }
            if pos == 0 && buf[0] == (n as i32) - 1 {
                break;
            }
        }
        sum
    }
}

fn main() {
    let n = 4;
    let res = Solution::total_n_queens(n);
    println!("There is {} solutions to {} queesn", res, n);

    let n = 8;
    let res = Solution::total_n_queens(n);
    println!("There is {} solutions to {} queesn", res, n);    
}


/*
The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.

Given an integer n, return the number of distinct solutions to the n-queens puzzle.
 */