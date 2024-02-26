use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    println!("{}", len);
}
