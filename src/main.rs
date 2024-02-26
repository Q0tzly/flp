use std::env;
use std::path::PathBuf;
use std::process;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";

const NAME: &str = "\x1b[0mGet Full Path Command\n\n";

fn to_help() -> String {
    format!(
        "{}{}help:\n  {}flp -h    {}put help",
        NAME, GREEN, CYAN, RESET
    )
}

fn help() {
    println!("{}{}Usage:\n  {}flp -h        {}put help\n  {}flp           {}put working dir\n  {}flp <PATH>    {}put full path of file or dir",
        NAME, GREEN, CYAN, RESET, CYAN, RESET, CYAN, RESET
    );
}

fn or_help(path: &str) {
    if path == "-h" || path == "--help" {
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
            println!("{}Not found that file or dir\n\n{}", RED, to_help());
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
