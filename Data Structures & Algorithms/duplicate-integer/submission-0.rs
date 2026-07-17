use std::collections::HashSet;
impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        // so we can use a HashSet and insert in it the value if not seen before if it is seen before return true immediately without 
        // doing another pass
        // complexity is Size -> O(n), and time is O(n)
        // if list is empty or has one element return false right away

        if nums.len() <= 1 {
            return false;
        }

        let mut set =  HashSet::new();
        for x in &nums {
            if set.contains(&x) {
                return true;
            }
            else {
                set.insert(x);
            }
        }

        return false
    } 
}
