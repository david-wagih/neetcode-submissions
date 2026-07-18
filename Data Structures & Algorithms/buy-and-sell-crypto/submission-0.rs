impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // prices = [10,1,5,6,7,1] this means the price of the stock at 0th day is 10 and then in 1st day is 1 and so on
        // best time to buy is on day 1 (value is 1) and sell it on day 4 (value of 7), the profit is 6 (the largest window between both)

        let mut min_so_far = prices[0];
        let mut best_profit = 0;

        for num in &prices {
            if *num < min_so_far {
                min_so_far = *num;
            }
            if *num - min_so_far > best_profit{
                best_profit = *num - min_so_far;
            }
        }
        return best_profit;
    }
}
