use std::env;

extern crate mknippo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let nippo_type = &args[1];
    let filename = format!("test_nippo_{}.txt", nippo_type);
    println!("{}", filename);
    // let filename = "post_test.txt";
    mknippo::run(&filename);
}

