use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for i in 0..nums.len() {
            if let Some(k) = map.get(&(target - nums[i])) {
                if *k != i {
                    return vec![*k as i32, i as i32];
                }
            }
            map.insert(nums[i], i);
        }
        panic!("not found!");
    }
}

struct Solution {}

fn main() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let ans = Solution::two_sum(nums, target);
    println!("ans = {:?}", ans);
}
