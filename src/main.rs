//! The simple python packaging tool ✨

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![feature(format_args_nl)]

mod errors;
mod package;

pub use errors::{Error, Result};
pub use package::{Package, PackageId};

use sqlx::SqlitePool;
use std::{env, process};

const DEFAULT_LOCKFILE: &str = "sqlite://nakes.lock";

/// Small integrated argument parser, this enum represents all possible commands
#[derive(Debug)]
enum ArgParse {
    Install,
    Uninstall,
}

/// Extra metadata for [ArgParse] enum
#[derive(Debug)]
struct ArgParseMeta {
    lockfile: Option<String>,
}

impl ArgParse {
    fn launch() -> (Self, ArgParseMeta) {
        let args: Vec<String> = env::args().collect();
        let get_arg_data = |name| {
            let ours = format!("--{}", name);
            let pos = match args.iter().position(|arg| arg == &ours) {
                Some(val) => val + 1,
                None => return None,
            };

            if args.len() < pos {
                eprintln!("{}No data given for {} argument", Self::err_msg(), ours);
                process::exit(1);
            } else if args[pos].starts_with("-") {
                eprintln!(
                    "{}Data for {} argument is an argument itself",
                    Self::err_msg(),
                    ours
                );
                process::exit(1);
            }

            Some(args[pos].clone())
        };

        if args.len() < 2 {
            eprintln!("{}Too few arguments", Self::err_msg());
            process::exit(1);
        } else if args.contains(&"--help".to_string()) {
            println!("{}", Self::help());
            process::exit(0);
        }

        let command = match args[1].as_str() {
            "install" => ArgParse::Install,
            "uninstall" | "remove" => ArgParse::Uninstall,
            unknown => {
                eprintln!("{}Unknown '{}' command", Self::err_msg(), unknown);
                process::exit(1)
            }
        };

        let args = ArgParseMeta {
            lockfile: get_arg_data("lockfile"),
        };

        (command, args)
    }

    fn help() -> String {
        const CLI_USAGE: &str = "nakes [COMMAND] [OPTIONS]";
        const CLI_DESCRIPTION: &str = "the simple python packaging tool ✨";

        format!("Usage: {}\n\nnakes\n  {}\n\nCOMMANDS:\n  install [pkg]     installs a package to venv\n  uninstall [pkg]   removes a package from venv\n  help              shows this message\n\nOPTIONS:\n  --lockfile [uri]  custom lockfile uri", CLI_USAGE, CLI_DESCRIPTION)
    }

    fn err_msg() -> String {
        format!("{}\n\nCMDERROR:\n", Self::help())
    }
}

/// Gets pool from args location
async fn get_pool(args: &ArgParseMeta) -> Result<SqlitePool> {
    SqlitePool::connect(
        args.lockfile
            .as_ref()
            .unwrap_or(&DEFAULT_LOCKFILE.to_string()),
    )
    .await
    .map_err(|err| Error::InvalidDatabase(err))
}

#[tokio::main]
async fn main() {
    let (command, args) = ArgParse::launch();
    let pool = get_pool(&args).await;

    println!("Command to do: {:?}", command);
    println!("Pool: {:?}", pool);
}
