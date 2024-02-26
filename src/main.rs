use std::env;
//use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    println!("{}", len);

    let path: Option<&str> = match len {
        1 => Some("."),
        2 => Some(args[1].as_str()),
        _ => None,
    };

    println!("{:?}", path);
}
