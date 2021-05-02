struct Solution{

}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n = nums1.len();
        let m = nums2.len();

        let half= (n + m)/2 + 1;

        let (mut pos, mut i, mut j) = (0, 0, 0);
        
        let mut current = 0;
        let mut previous = 0;
        while pos<half {
            // advance
            if i >= n {
                previous = current;
                current = nums2[j];
                j += 1;
            } else if j >= m {
                previous = current;
                current = nums1[i];
                i += 1;
            } else {
                if nums1[i] <= nums2[j] {
                    previous = current;
                    current = nums1[i];
                    i += 1;
                } else {
                    previous = current;
                    current = nums2[j];
                    j += 1;
                }
            }
            pos += 1;
        };
        if (n+m)%2==1 {
            return current as f64;
        } else {
            return ((current + previous) as f64)/2.0;
        }
    }
}

fn main() {
    let arr1 = vec![1, 3, 5, 7, 9];
    let arr2 = vec![2, 4, 6, 8, 10];

    let res = Solution::find_median_sorted_arrays(arr1, arr2);
    println!("The median of two array is: {}", res);

    let arr1 = vec![1, 3];
    let arr2 = vec![2];

    let res = Solution::find_median_sorted_arrays(arr1, arr2);
    println!("The median of two array is: {}", res);    
}
