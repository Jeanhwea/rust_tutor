use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache: HashMap<i32, i32> = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            if let Some(&j) = cache.get(&(target - *v)) {
                return vec![i as i32, j as i32];
            }
            cache.insert(*v as i32, i as i32);
        }

        panic!("NOT_FOUND");
    }
}

struct Solution {}

fn main() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let ans = Solution::two_sum(nums, target);
    println!("ans = {:?}", ans);
}
