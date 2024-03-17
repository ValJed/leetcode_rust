struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let (output, _): (Vec<i32>, i32) = nums.iter().fold((vec![], 0), |mut acc, num| {
            acc.1 += num;
            acc.0.push(acc.1);

            acc
        });

        output
    }
}

fn main() {
    let input = vec![1, 2, 3, 4];
    let res = Solution::running_sum(input);

    println!("res: {:?}", res);
}
