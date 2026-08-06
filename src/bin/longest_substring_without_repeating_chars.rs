use std::collections::HashSet;

pub fn run(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut max = 0;
    let mut left = 0;
    let mut seen = HashSet::new();

    for right in 0..chars.len() {
        while seen.contains(&chars[right]) {
            seen.remove(&chars[left]);
            left += 1;
        }

        seen.insert(&chars[right]);

        let cur = right - left + 1;
        if cur > max {
            max = cur
        }
    }

    max as i32
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(String::from("abcabcbb")), 3);
        assert_eq!(run(String::from("bbbbb")), 1);
        assert_eq!(run(String::from("pwwkew")), 3);
        assert_eq!(run(String::from("S")), 1);
        assert_eq!(run(String::from("1R1T7")), 3);
    }
}
