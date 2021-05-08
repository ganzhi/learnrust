struct Solution {

}

impl Solution {
    pub fn jump_internal(nums: &Vec<i32>, init:usize, buf: &mut Vec<i32>) -> i32 {
        println!("Init is {}", init);
        if buf[init] != -1 {
            return buf[init];
        }

        if init == nums.len()-1 {
            return 0;
        }
        let max_step = nums[init];
        if max_step <= 0 {
            return 10000;
        }
        if init + (max_step as usize) >= nums.len()-1 {
            buf[init] = 1;
            return 1;
        }
        let mut min = 10000;
        for i in 1..(max_step+1) {
            let child = Solution::jump_internal(nums, init + (i as usize), buf);
            if child < min {
                min = child;
            }
        }
        buf[init] = min +1;
        return min + 1;
    }

    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut buf = vec![-1; nums.len()];
        Solution::jump_internal(&nums, 0, &mut buf)
    }
}

fn main() {
    let nums = vec![2,3,0,1,4];
    let result = Solution::jump(nums);
    println!("Hello, world: {}", result);

    let nums = vec![2,1];
    let result = Solution::jump(nums);
    println!("Hello, world: {}", result);

    let nums = vec![5,6,4,4,6,9,4,4,7,4,4,8,2,6,8,1,5,9,6,5,2,7,9,7,9,6,9,4,1,6,8,8,4,4,2,0,3,8,5];
    let result = Solution::jump(nums);
    println!("Hello, world: {}", result);        
}
