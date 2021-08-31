
struct Solution{

}

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        if fruits.len() == 0 {
            return 0;
        }
        
        let mut fruit1 = fruits[0];
        let mut fruit2 = -1;        
        let mut fruit1_start = 0;
        let mut fruit2_start = 0;
        let mut max_length = 1;
        let mut curr_length = 1;
        let mut pos = 1usize;
        loop{
            if pos >= fruits.len(){
                if curr_length > max_length {
                    max_length = curr_length;
                }
                break;
            }
            //println!("Pos {} Curr_length {} f1 {} f1start {} f2 {} f2start {}", pos, curr_length, fruit1, fruit1_start, fruit2, fruit2_start);
            let item = fruits[pos];

            if item == fruit1 || item == fruit2 {
                curr_length += 1;
                if item == fruit1 {
                    fruit1_start = pos + 1;
                } else {
                    fruit2_start = pos + 1;
                }
                pos += 1;
                continue;
            }
            if fruit2 == -1 {
                fruit2 = item;
                curr_length += 1;
                fruit1_start = pos;
                fruit2_start = pos + 1;
                pos += 1;
                continue;
            }
            if curr_length > max_length {
                max_length = curr_length;
            }
            if fruits[pos-1] == fruit2 {
                pos = fruit1_start;                
                fruit1 = fruit2;
                fruit1_start = fruit2_start;
            } else {
                pos = fruit2_start;
            }
            fruit2 = item;
            fruit2_start = pos + 1;
            curr_length = 0;
        }
        return max_length;
    }
}

fn main() {
    let fruits = vec![3,3,3,1,2,1,1,2,3,3,4];
    let res = Solution::total_fruit(fruits);
    println!("The result is {}", res);
    let fruits = vec![1, 2, 1];
    let res = Solution::total_fruit(fruits);
    println!("The result is {}", res);    
    let fruits = vec![1,0,1,4,1,4,1,2,3];
    let res = Solution::total_fruit(fruits);
    println!("The result is {}", res);
    let fruits = vec![1,0,29,29,29,29,29,29,0,0,29,8,8,29,8,29,8,8,15,8,8,15,15,8,15,15,8,8,7,5];
    let res = Solution::total_fruit(fruits);
    println!("The result is {}", res);
}
