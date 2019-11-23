// https://qiita.com/mattn/items/ae764601862b0073071e
extern crate rand;

use rand::seq::SliceRandom;

fn main() {
    let mut v = vec!["ボ", "ジョ", "レ", "ヌ", "ボ"];
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    let s: String = v.iter().enumerate().map(|(i, str)| if i > 1 { str.to_string() + "ー" } else { str.to_string() }).collect();
    println!("{}", s);
}
