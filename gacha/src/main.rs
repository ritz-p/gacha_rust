use dotenv::dotenv;
use gacha::gacha;
use std::env;
mod gacha;
fn main() {
    dotenv().ok();
    let csv = env::var("CSV").expect("csv file path must be set");
    for i in 0..10{
        let introduction = i.to_string() + "回目ガチャ結果";
        let gacha_res = gacha(&csv);
        println!("{}",introduction);
        println!("{}",gacha_res);
    }
}

