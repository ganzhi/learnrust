
struct Solution {

}

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut count = 0;
        let mut pos = 0;
        for i in 0..(nums.len()-1) {
            if nums[i] > nums[i+1] {
                count += 1;
                pos = i
            }
            if count > 1 {
                return false;
            }
        }
        if count == 1 {
            println!("Pos is {}", pos);
            if pos == 0 || pos == nums.len()-2 {
                return true;
            }

            if nums[pos-1] <= nums[pos+1]{
                return true;
            }

            if nums[pos] <= nums[pos+2] {
                return true;
            }

            return false;
        }
        true
    }
}

fn main() {
    let test = vec![-1,4,2,3];
    let result  = Solution::check_possibility(test);

    println!("It's possible or not?: {}", result);

    let test = vec![5, 7, 1, 8];
    let result  = Solution::check_possibility(test);

    println!("It's possible or not?: {}", result);    
}
