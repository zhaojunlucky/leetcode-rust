 fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut min_price:usize = 0;

    for i in 1..prices.len() {
        if prices[i] < prices[min_price] {
            min_price = i;
        }
        max_profit = max_profit.max(prices[i] - prices[min_price]);

    }

    max_profit
}

#[cfg(test)]
mod tests {
    fn _max_profit(prices: Vec<i32>, expected: i32) {
        assert_eq!(super::max_profit(prices), expected);
    }

    #[test]
    fn test_max_profit() {
        _max_profit(vec![7,1,5,3,6,4], 5);
        _max_profit(vec![7,6,4,3,1], 0);
    }
}