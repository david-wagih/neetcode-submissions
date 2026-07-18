impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // we can use binary search as it is a sorted list of integers, so we can get the middle value and compare the target with it
        let mut low: i32 = 0;
        let mut high: i32 = nums.len() as i32 - 1;

        while low <= high {
            let mid = low + (high - low) / 2;

            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] < target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        -1
    }
}
