use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数が一つではありません");
    }

    let json = args[1].as_str();

    match json {
        "null" => println!("null"),
        "true" => println!("true"),
        "false" => println!("false"),
        _ => panic!("入力値はnull、true、falseのどれかにしてください"),
    }
}
