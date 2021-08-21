struct Solution {

}

impl Solution {
    fn clear(grid: &mut Vec<Vec<char>>, i: usize, j:usize) {
        let m = grid.len();
        let n = grid[0].len();

        grid[i][j] = '0';
        if (i as i32) -1 >=0 && grid[i-1][j] == '1' {
            Solution::clear(grid, i-1, j);
        }

        if i+1 < m && grid[i+1][j]== '1' {
            Solution::clear(grid, i+1, j);
        }

        if j+1 < n && grid[i][j+1] == '1' {
            Solution::clear(grid, i, j+1);
        }

        if (j as i32) -1 >=0 && grid[i][j-1] == '1' {
            Solution::clear(grid, i, j-1);
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut gridc = grid.clone();
        let m = gridc.len();
        let n = gridc[0].len();

        let mut sum = 0;
        for i in 0..m {
            for j in 0..n {
                if gridc[i][j] == '1' {
                    sum += 1;                    
                    Solution::clear(&mut gridc, i, j);
                }
            }
        }
        
        sum
    }
}

fn main() {
    // let grid = vec![
    //     vec!['1','1','1','1','0'],
    //     vec!['1','1','0','1','0'],
    //     vec!['1','1','0','0','0'],
    //     vec!['0','0','0','0','0']
    // ];

    let grid = vec![
        vec!['1','1','0','0','0'],
        vec!['1','1','0','0','0'],
        vec!['0','0','1','0','0'],
        vec!['0','0','0','1','1']];

    let res = Solution::num_islands(grid);
    println!("Num of islands: {}", res);

    let grid = vec![
        vec!['1','0','1','1','1'],
        vec!['1','0','1','0','1'],
        vec!['1','1','1','0','1']];
    let res = Solution::num_islands(grid);
    println!("Num of islands: {}", res);        
}
