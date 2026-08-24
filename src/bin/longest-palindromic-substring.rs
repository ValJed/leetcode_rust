fn main() {
    let input = String::from("aba");
    let res = run(input);

    println!("res: {:?}", res);
}

pub fn run(s: String) -> String {
    let len = s.len();
    let mut best = String::new();
    let mut best_size: usize = 0;

    let chars: Vec<char> = s.chars().collect();

    for (i, _) in chars.iter().enumerate() {
        let mut cur = String::new();

        if len - i <= best_size {
            continue;
        }

        let slice = &chars[i..len];
        for sub_char in slice {
            cur.push(sub_char.clone());

            if cur.len() <= best_size {
                continue;
            }

            let reversed = cur.chars().rev().collect::<String>();
            if cur == reversed && cur.len() > best_size {
                best = cur.clone();
                best_size = cur.len();
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(String::from("a")), String::from("a"));
        assert_eq!(run(String::from("babad")), String::from("bab"));
        assert_eq!(run(String::from("cbbd")), String::from("bb"));
        assert_eq!(run(String::from("aba")), String::from("aba"));
    }
}
