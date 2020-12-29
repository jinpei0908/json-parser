use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数が一つじゃありません");
    }

    println!("{:?}", args);
}
