struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().fold(0, |acc, customer| {
            let sum_customer: i32 = customer.iter().sum();
            if sum_customer > acc {
                return sum_customer;
            }

            acc
        })
    }
}

fn main() {
    let input = vec![vec![1, 2, 3], vec![3, 2, 1]];
    let res = Solution::maximum_wealth(input);

    println!("res: {:?}", res);
}
