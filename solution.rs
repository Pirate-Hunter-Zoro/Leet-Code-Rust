use std::collections::HashMap;

impl Solution {

    ////////////////////////////////////////////////////////////////////////////////////////////

    /*
    Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
    You may assume that each input would have exactly one solution, and you may not use the same element twice.
    You can return the answer in any order.

    Link: https://leetcode.com/problems/two-sum/description/
     */
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create a map to store the indices and values of the numbers
        let mut map: HashMap<i32, i32> = HashMap::new();

        // Iterate through the numbers
        for (i, &num) in nums.iter().enumerate() {
            // Calculate the difference between the target and the current number
            let diff: i32 = target - num;
            if let Some(&j) = map.get(&diff) {
                // If the difference is found in the map, return the indices
                return vec![j, i as i32];
            }
            // Store this number and its index in the map
            map.insert(num, i as i32);
        }

        return vec![]
    }

    ////////////////////////////////////////////////////////////////////////////////////////////
    
    

}