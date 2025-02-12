pub struct  MaxProfit;
impl MaxProfit {
    pub fn get_profit (self, prices: Vec<i32> ) -> i32 {
     
        let mut max_profit = 0;
        let n = prices.len() - 1;
        for i in 0..n {
            if prices[i + 1] > prices[i] {
                max_profit += prices[i+1] - prices[i];
                
            }
        }

        return max_profit;
    }
}