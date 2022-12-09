use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for a in args {
        if a == "1" {
            panic!("failed")
        }
        println!("{}", a)
    }
}