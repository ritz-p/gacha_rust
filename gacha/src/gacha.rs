extern crate rand;
extern crate csv;
use rand::Rng;
use serde_derive::{Deserialize,Serialize};
use std::fs;
use std::io;
pub fn gacha(path: &str) -> String{
    let content = fopen(path);
    let gifts = parse(content.unwrap()).unwrap();
    let mut rng = rand::thread_rng();
    let mut sum:usize = 0;
    for gift in &gifts{
        sum += gift.weight;
    }

    let rand = rng.gen_range(0..sum);
    let mut res = "".to_string();
    for gift in &gifts{
        sum -= gift.weight;
        if sum < rand{
            res = gift.name.clone();
            break;
        }
    }
    res
}

fn fopen(path: &str) -> Result<String,io::Error>{
    let content = fs::read_to_string(path);
    content
}

fn parse(content: String) -> Result<Vec<GachaElement>,io::Error>{
    let mut gifts:Vec<GachaElement> = vec![];
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(content.as_bytes());
    for res in reader.deserialize(){
        let record:GachaElement = res?;
        gifts.push(record);
    }
    Ok(gifts)
}


#[derive(Debug, Deserialize, Serialize)]
pub struct GachaElement{
    name: String,
    weight: usize,
}

enum Rarerity{
    Garbage,
    Common,
    UnCommon,
    Rare,
    SuperRare,
    UltraRare
}