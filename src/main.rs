use std::env;
use std::path::Path;
use std::process;

const HELP: &str = concat!(
    "Get Full Path Command\n\n",
    "\x1b[1m\x1b[32mUsage:\x1b[0m\n",
    "  \x1b[36mflp -h, --help     \x1b[0mput help\n",
    "  \x1b[36mflp -v, --version  \x1b[0mput version\n",
    "  \x1b[36mflp                \x1b[0mput working dir\n",
    "  \x1b[36mflp <PATH>         \x1b[0mput full path of file or dir"
);

const USAGE_SUFFIX: &str =
    "\n\n\x1b[1m\x1b[32mUsage:\x1b[0m\n  \x1b[36mflp -h, --help     \x1b[0mput help";

#[derive(Debug)]
enum AppError {
    TooManyArgs,
    NotFound { path: String },
}

impl AppError {
    fn message(&self) -> String {
        match self {
            AppError::TooManyArgs => format!(
                "\x1b[31m\x1b[1mError: \x1b[0mToo many arguments provided.{}",
                USAGE_SUFFIX
            ),
            AppError::NotFound { path } => format!(
                "\x1b[31m\x1b[1mError: \x1b[0mNo such file or directory: {}{}",
                path,
                USAGE_SUFFIX
            ),
        }
    }
}

fn resolve_path(input: &str) -> Result<String, AppError> {
    let path = Path::new(input);
    path.canonicalize()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|_| AppError::NotFound {
            path: input.to_string(),
        })
}

fn run() -> Result<Option<String>, AppError> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => resolve_path(".").map(Some),
        2 => {
            let arg = &args[1];
            match arg.as_str() {
                "-h" | "--help" => {
                    println!("{}", HELP);
                    Ok(None)
                }
                "-v" | "--version" => {
                    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                    Ok(None)
                }
                _ => resolve_path(arg).map(Some),
            }
        }
        _ => Err(AppError::TooManyArgs),
    }
}

fn main() {
    match run() {
        Ok(Some(output)) => println!("{}", output),
        Ok(None) => {}
        Err(e) => {
            eprintln!("{}", e.message());
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_current_dir() {
        let result = resolve_path(".");
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn resolve_nonexistent() {
        let result = resolve_path("/this/should/not/exist");
        assert!(
            matches!(result, Err(AppError::NotFound { path }) if path == "/this/should/not/exist")
        );
    }

    #[test]
    fn error_message_contents() {
        let err = AppError::TooManyArgs;
        assert!(err.message().contains("Too many arguments"));

        let err = AppError::NotFound {
            path: "foo".to_string(),
        };
        assert!(err.message().contains("foo"));
    }
}
