struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        if x < 10 {
            return true;
        }

        let reversed = x.to_string().chars().rev().collect::<String>();
        if x.to_string() == reversed {
            return true;
        }

        false
    }
}
fn main() {
    let input = 121;
    let res = Solution::is_palindrome(input);

    println!("res: {:?}", res);
}
