// Category: algorithms
// Level: Easy
// Percent: 50.46485%

// Given an array of integers nums and an integer target, return indices of the
// two numbers such that they add up to target.
//
// You may assume that each input would have exactly one solution, and you may
// not use the same element twice.
//
// You can return the answer in any order.
//
//
// Example 1:
//
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//
//
// Example 2:
//
// Input: nums = [3,2,4], target = 6
// Output: [1,2]
//
//
// Example 3:
//
// Input: nums = [3,3], target = 6
// Output: [0,1]
//
//
//
// Constraints:
//
//
//      2 <= nums.length <= 10⁴
//      -10⁹ <= nums[i] <= 10⁹
//      -10⁹ <= target <= 10⁹
//      Only one valid answer exists.
//
//
//
// Follow-up: Can you come up with an algorithm that is less than O(n²) time
// complexity?

// SOLUTION_BEGIN
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (i, v) in nums.iter().enumerate() {
            match cache.get(&(target - *v)) {
                Some(&j) => return vec![i as i32, j as i32],
                None => {}
            }
            cache.insert(*v as i32, i as i32);
        }

        panic!("NOT_FOUND");
    }
}
// SOLUTION_END

impl Solution {
    pub fn two_sum_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
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

    let ans = Solution::two_sum(nums.clone(), target);
    println!("ans = {:?}", ans);

    let ans2 = Solution::two_sum_2(nums.clone(), target);
    println!("ans2 = {:?}", ans2);
}
