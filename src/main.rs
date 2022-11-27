// use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "test_nippo.txt";
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("With text:\n{}", contents);

    print_nippo_start();
}

fn print_nippo_start() {
    println!("{}", construct_mention_raw(&["山", "島", "swamp", "平地"]).unwrap());
    println!("■本日の予定■");
    println!("（2021/10/27）11:30～20:00勤務予定");
    println!("◇APOCALYPSE");
    println!("・ APOCALYPSE-5 レビュー集計 [次回11/1]");
    println!("・ APOCALYPSE-12 タグ設置 [顧客確認中]");
    println!("・ ローカル環境構築[作業中] :triplets_parrot:");
}

fn construct_mention_name(name: &str) -> Result<String, String>{
    Ok('@'.to_string() + name)
}

fn construct_mention_raw(names: &[&str]) -> Result<String, String> {
    let mut return_raw_string = String::new();
    for name in names.iter() {
        if return_raw_string.len() != 0 {
            return_raw_string.push(' ');
        }
        return_raw_string.push_str(&construct_mention_name(name).unwrap());
    }
    Ok(return_raw_string)
}

#[cfg(test)]

#[test]
fn construct_mention_test() {
    assert_eq!(construct_mention_name("abc").unwrap(), "@abc");
    assert_eq!(construct_mention_name("試験山").unwrap(), "@試験山");
}
#[test]
fn construct_mention_raw_test() {
    assert_eq!(construct_mention_raw(&["abc"]).unwrap(), "@abc");
    assert_eq!(construct_mention_raw(&["abc", "cbd"]).unwrap(), "@abc @cbd");
    assert_eq!(construct_mention_raw(&["試験山", "abc", "cbd"]).unwrap(), "@試験山 @abc @cbd");
}
