fn calculate_average(prices: &Vec<f64>) -> f64 {
    let sum: f64 = prices.iter().sum();
    sum / prices.len() as f64
}

fn main() {
    let stock_prices = vec![50.0, 52.5, 49.8, 53.2, 48.5, 51.0, 50.5, 52.0, 49.5, 50.2];
    let n = 5;

    let mut recent_prices: Vec<f64> = Vec::new();

    for price in stock_prices {
        recent_prices.push(price);

        if recent_prices.len() > n {
            recent_prices.remove(0);
        }

        let average_price = calculate_average(&recent_prices);

        println!(
            "Real-time Average Stock Price for the last {} updates: {:.2}",
            recent_prices.len(),
            average_price
        );
    }
}
