use std::env;
use std::path::PathBuf;
use std::process;

fn to_help() -> String {
    format!("help:\n  flp -h")
}

fn help() {
    println!("help:\n  flp -h    put help\n  flp    put working dir\n  flp [file or dir path]    put full path of file or dir");
}

fn or_help(path: &str) {
    if path == "-h" {
        help();
        process::exit(0);
    }
}

fn get_fpath(path: &str) -> String {
    or_help(path);
    let path = PathBuf::from(path);
    match path.canonicalize() {
        Ok(fpath) => fpath.to_string_lossy().to_string(),
        Err(_) => {
            println!("Not found that file or dir\n{}", to_help());
            process::exit(0);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    let path: Option<&str> = match len {
        1 => Some("."),
        2 => Some(args[1].as_str()),
        _ => None,
    };

    let fpath: String = match path {
        Some(path) => get_fpath(path),
        None => to_help(),
    };

    println!("{}", fpath);
}
