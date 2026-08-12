fn main() {
    let nums1 = vec![0, 0, 0, 0, 0];
    let nums2 = vec![-1, 0, 0, 0, 0, 0, 1];

    run(nums1, nums2);
}

fn run(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    let total_len = nums1.len() + nums2.len();
    let middle_dec = total_len as f64 / 2.0 as f64;
    let is_even = middle_dec.fract() == 0.0;
    let mut merged: Vec<i32> = vec![];
    let stop = if is_even {
        (middle_dec.ceil() + 1.0) as usize
    } else {
        middle_dec.ceil() as usize
    };

    let result: f64;
    for _ in 1..stop + 1 {
        match (nums1.get(0), nums2.get(0)) {
            (None, None) => {}
            (None, Some(_)) => {
                let value = nums2.remove(0);
                merged.push(value);
            }
            (Some(_), None) => {
                let value = nums1.remove(0);
                merged.push(value);
            }
            (Some(v1), Some(v2)) => {
                if v1 > v2 {
                    let value = nums2.remove(0);
                    merged.push(value);
                } else {
                    let value = nums1.remove(0);
                    merged.push(value);
                }
            }
        }
    }

    let last_i = merged.len() - 1;
    if is_even {
        let last = merged[last_i] as f64;
        let first = merged[last_i - 1] as f64;
        result = (first + last) / 2.0;
    } else {
        result = merged[last_i] as f64
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(vec![1, 3], vec![2]), 2.0);
        assert_eq!(run(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(run(vec![0, 0, 0, 0, 0], vec![-1, 0, 0, 0, 0, 0, 1]), 0.0);
    }
}
