use std::env;
use std::fmt;
use std::path::Path;
use std::process;

enum Command {
    Output(String),
    Help,
    Version,
}

#[derive(Debug)]
enum AppError {
    TooManyArgs,
    NotFound { path: String },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = env!("CARGO_PKG_NAME");
        match self {
            AppError::TooManyArgs => write!(
                f,
                "\x1b[31m\x1b[1mError: \x1b[0mToo many arguments provided.\n\n\
                 \x1b[1m\x1b[32mUsage:\x1b[0m\n  \x1b[36m{} -h, --help     \x1b[0mput help",
                name
            ),
            AppError::NotFound { path } => write!(
                f,
                "\x1b[31m\x1b[1mError: \x1b[0mNo such file or directory: {}\n\n\
                 \x1b[1m\x1b[32mUsage:\x1b[0m\n  \x1b[36m{} -h, --help     \x1b[0mput help",
                path,
                name
            ),
        }
    }
}

fn help_text() -> String {
    let name = env!("CARGO_PKG_NAME");
    format!(
        "Get Full Path Command\n\n\
         \x1b[1m\x1b[32mUsage:\x1b[0m\n\
          \x1b[36m{} -h, --help     \x1b[0mput help\n\
          \x1b[36m{} -v, --version  \x1b[0mput version\n\
          \x1b[36m{}                \x1b[0mput working dir\n\
          \x1b[36m{} <PATH>         \x1b[0mput full path of file or dir",
        name, name, name, name,
    )
}

fn resolve_path(input: &str) -> Result<String, AppError> {
    Path::new(input)
        .canonicalize()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|_| AppError::NotFound {
            path: input.to_string(),
        })
}

fn run(args: &[String]) -> Result<Command, AppError> {
    match args.len() {
        1 => resolve_path(".").map(Command::Output),
        2 => {
            let arg = &args[1];
            match arg.as_str() {
                "-h" | "--help" => Ok(Command::Help),
                "-v" | "--version" => Ok(Command::Version),
                _ => resolve_path(arg).map(Command::Output),
            }
        }
        _ => Err(AppError::TooManyArgs),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match run(&args) {
        Ok(Command::Output(s)) => println!("{}", s),
        Ok(Command::Help) => println!("{}", help_text()),
        Ok(Command::Version) => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        }
        Err(e) => {
            eprintln!("{}", e);
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
    fn run_default() {
        assert!(matches!(
            run(&["flp".to_string()]),
            Ok(Command::Output(_))
        ));
    }

    #[test]
    fn run_help() {
        assert!(matches!(
            run(&["flp".to_string(), "--help".to_string()]),
            Ok(Command::Help)
        ));
    }

    #[test]
    fn run_version() {
        assert!(matches!(
            run(&["flp".to_string(), "--version".to_string()]),
            Ok(Command::Version)
        ));
    }

    #[test]
    fn run_too_many_args() {
        assert!(matches!(
            run(&["flp".to_string(), "a".to_string(), "b".to_string()]),
            Err(AppError::TooManyArgs)
        ));
    }

    #[test]
    fn display_error_too_many_args() {
        let err = AppError::TooManyArgs;
        let msg = err.to_string();
        assert!(msg.contains("Too many arguments"));
        assert!(msg.contains("Usage:"));
    }

    #[test]
    fn display_error_not_found() {
        let err = AppError::NotFound {
            path: "foo".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("foo"));
        assert!(msg.contains("Usage:"));
    }
}
