pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // solve this function based on test below
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    // write another test
    #[test]
    fn example_2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    // write another test
    #[test]
    fn example_3() {
        assert_eq!(two_sum(vec![3, 3], 6), [0, 1]);
    }
}
