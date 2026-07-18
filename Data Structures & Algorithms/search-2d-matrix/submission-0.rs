impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // each row is sorted ascendingly
        // the first integer in a row is bigger than the last integer in the previous row
        // return true if target exists and false otherwise

        // we can do Binary Search here but the diff it is a 2D matrix not just a list
        // loop over rows and columns but surely we need to start with comparing the middle to the target and based on that shift the left or right
        // pointers
        // if we can flatten this 2D matrix to one long 1D array that has (x,y0) coordinate
        let rows = matrix.len();
        if rows == 0 {
            return false;
        }
        let cols = matrix[0].len();
        if cols == 0 {
            return false;
        }

        let mut low: i32 = 0;
        let mut high: i32 = (rows * cols) as i32 - 1;

        while low <= high {
            let mid = low + (high - low) / 2;
            let row = (mid as usize) / cols;
            let col = (mid as usize) % cols;
            let mid_val = matrix[row][col];

            if mid_val == target {
                return true;
            } else if mid_val < target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        false

    }
}
