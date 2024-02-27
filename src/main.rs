use std::env;
use std::path::PathBuf;
use std::process;

const NO_SUCH: &str = "\x1b[31m\x1b[1mHelp: \x1b[0m\x1b[0mNo such file or directory.";
const MANY_ARGS: &str = "\x1b[31m\x1b[1mHelp: \x1b[0m\x1b[0mToo many arguments provided.";

const TO_HELP: &str =
    "\n\x1b[1m\x1b[32mUsage:\x1b[0m\n  \x1b[36mflp -h, --help     \x1b[0mput help\x1b[0m";

const VERSION: &str = "flp    0.1.0 (2024-02-27)";

const HELP: &str =
    "\x1b[0mGet Full Path Command\n\n\x1b[1m\x1b[32mUsage:\x1b[0m\n  \x1b[36mflp -h, --help     \x1b[0mput help\n  \x1b[36mflp -v, --version  \x1b[0mput version\n  \x1b[36mflp                \x1b[0mput working dir\n  \x1b[36mflp <PATH>         \x1b[0mput full path of file or dir";

fn or_options(path: &str) {
    let res: Option<&str> = match path {
        "-h" | "--help" => Some(HELP),
        "-v" | "--version" => Some(VERSION),
        _ => None,
    };

    if let Some(res) = res {
        println!("{}", res);
        process::exit(0);
    }
}

fn get_fpath(path: &str) -> String {
    let path = PathBuf::from(path);
    match path.canonicalize() {
        Ok(fpath) => fpath.to_string_lossy().to_string(),
        Err(_) => {
            println!("{}{}", NO_SUCH, TO_HELP);
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
        Some(path) => {
            or_options(path);
            get_fpath(path)
        }
        None => format!("{}{}", MANY_ARGS, TO_HELP),
    };

    println!("{}", fpath);
}
