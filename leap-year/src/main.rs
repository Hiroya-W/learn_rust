// P.217 うるう年と平年
// 入力された年がうるう年かどうかを判断するプログラム

use std::io;
use std::io::Write;

fn main() {
    let mut year = String::new();
    print!("Please input a year to check if it is a leap year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();
    let year = year.trim().parse::<u32>().unwrap();

    if is_leap_year(year) {
        println!("{} is a leap year!", year);
    } else {
        println!("{} is not a leap year.", year);
    }
}

fn is_leap_year(year: u32) -> bool {
    // 西暦が4で割り切れる年をうむう年とする
    //1の例外として、西暦が100で割り切れて400で割り切れない年は平年とする
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}
