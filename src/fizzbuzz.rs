struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..n + 1)
            .map(|num| {
                if num % 3 == 0 && num % 5 == 0 {
                    return String::from("FizzBuzz");
                }
                if num % 3 == 0 {
                    return String::from("Fizz");
                }
                if num % 5 == 0 {
                    return String::from("Buzz");
                }

                num.to_string()
            })
            .collect()
    }
}

fn main() {
    let input = 15;
    let res = Solution::fizz_buzz(input);

    println!("res: {:?}", res);
}
