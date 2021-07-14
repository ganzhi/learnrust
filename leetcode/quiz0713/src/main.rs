
struct Solution {

}

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut l = 0 as usize;
        if nums[0] > nums[1] {
            return 0;
        }
        let mut r = nums.len() - 1;
        if nums[r] > nums[r-1] {
            return r as i32;
        }
        loop{
            let mid = (l+r)/2;
            if nums[mid] > nums[mid-1] && nums[mid] > nums[mid+1] {
                return mid as i32;
            }

            if nums[mid] > nums[mid-1] {
                l = mid;
            } else {
                r = mid;
            }
        }
    }
}

fn main() {
    let v = vec![1,2,3, 1];
    let r = Solution::find_peak_element(v);
    println!("Hello, world! {}", r);
}
