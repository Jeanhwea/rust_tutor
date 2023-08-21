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

    pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
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

struct Solution {}

fn main() {
    let nums = vec![3, 2, 4];
    let target = 6;

    let ans = Solution::two_sum(nums.clone(), target);
    println!("ans = {:?}", ans);

    let ans2 = Solution::two_sum_2(nums.clone(), target);
    println!("ans2 = {:?}", ans2);
}
