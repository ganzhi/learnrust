struct Solution {

}

impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut e: Vec<&Vec<i32>> = courses.iter().filter(|e| e[1] >= e[0]).collect();
        e.sort_unstable_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());
        //println!("Sorted: {:?}", e);

        let mut current_day = 0;
        let mut result = 0;
        let mut scheduled = Vec::<Vec::<i32>>::new();
        for item in e {
            if current_day + item[0] > item[1] {
                let mut max_dur = item[0];
                let mut max_loc = 0;
                for (i, v) in scheduled.iter().enumerate() {
                    if v[0] > max_dur {
                        max_dur = v[0];
                        max_loc = i;
                    }
                }
                if max_dur > item[0] {
                    current_day -= max_dur - item[0];
                    scheduled[max_loc][0] = item[0];
                    scheduled[max_loc][1] = item[1];
                }
            } else {
                result += 1;
                current_day += item[0];
                scheduled.push(item.clone());
            }
        }
        
        //println!("{:?}", scheduled);
        return result;
    }
}

fn main() {
    let course = vec![vec![4,3],vec![3,2]];
    let res = Solution::schedule_course(course);
    println!("You can do {} courses", res);

    let course = vec![vec![1,2],vec![1,1]];
    let res = Solution::schedule_course(course);
    println!("You can do {} courses", res);

    let course = vec![[860,4825],[13,1389],[746,8823],[455,2778],[233,2069],[106,5648],[802,2969],[958,2636],[567,2439],[623,1360],[700,4206],[9,3725],[241,7381]];
    let course: Vec<Vec<i32>> = course.iter().map(|e| e.to_vec()).collect();
    let res = Solution::schedule_course(course);
    println!("You can do {} courses", res);

    let course = vec![[7,17],[3,12],[10,20],[9,10],[5,20],[10,19],[4,18]];
    let course: Vec<Vec<i32>> = course.iter().map(|e| e.to_vec()).collect();
    let res = Solution::schedule_course(course);
    println!("You can do {} courses", res);    
}
