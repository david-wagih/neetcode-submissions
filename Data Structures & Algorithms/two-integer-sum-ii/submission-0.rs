impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        // the list is sorted in ascending order
        // one valid solution
        // must use O(1) space -> no HashMap should be used
        // 1-indexed not 0-indexed
        // we willl be using two pointers, one to the first element the smallest and one at the last element which is the larget
        // if the sum is < target then increment the left pointer to increase the sum in next step
        // if sum > target move the right one to the inside towards the left
        // when we reach the sum return both indices plus 1 to each to be 1-indexed
        // when left = right then we have no solutions

        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            if numbers[left] + numbers[right] > target {
                right = right - 1;
            }
            else if  numbers[left] + numbers[right] < target {
                left = left + 1;
            }
            else { 
                 return vec![(left + 1) as i32, (right + 1) as i32];
            }
        }
        unreachable!()

    }
}
