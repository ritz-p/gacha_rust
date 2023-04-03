use dotenv::dotenv;
use gacha::gacha;
use std::env;
mod gacha;
fn main() {
    dotenv().ok();
    let csv = env::var("CSV").expect("csv file path must be set");
    for _ in 0..10{
        let gacha_res = gacha(&csv);
        println!("{}",gacha_res);
    }
}

