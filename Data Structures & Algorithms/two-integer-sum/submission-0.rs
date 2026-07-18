use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // we can use a HashMap to store value -> index
        // so for each element in the list we ask ourselves a question if target - current_number exists in the map, if yes return both indices.
        // if not insert this element in the map with its index

        let mut map = HashMap::new();

        for (index, value) in nums.iter().enumerate() {
            if map.contains_key(&(target - *value)) {
                // return the index of this value in the map with the index of the current value
                let found_index = map.get(&(target - *value)).unwrap();
                return vec![*found_index as i32, index as i32];
            } else {
                // insert this element in the map with its index
                map.insert(*value, index);
            }
        }
        unreachable!()
    }
}
