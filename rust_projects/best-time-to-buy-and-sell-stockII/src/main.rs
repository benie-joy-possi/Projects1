use max_profit::MaxProfit;
fn main() {
    let max_profit = MaxProfit::get_profit(MaxProfit, vec![7,6,4,3,1]);
    println!("the profit  is: {}", max_profit );
}
mod max_profit;