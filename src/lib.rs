use clap::{Parser, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Type {
    #[value(name = "f")]
    File,
    #[value(name = "d")]
    Dir,
    #[value(name = "l")]
    Link,
}

#[derive(Parser, Debug)]
#[command(name = "findr")]
#[command(version = "0.1.0")]
#[command(about = "Rust find")]
#[command(author = "Radish-Miyazaki <y.hidaka.kobe@gmail.com>")]
pub struct Args {
    #[arg(value_name = "PATH", default_value = ".", help = "Search paths")]
    paths: Vec<String>,
    #[arg(value_name = "NAME", short, long = "name", help = "Name", num_args = 1..)]
    names: Vec<Regex>,
    #[arg(value_name = "TYPE", short = 't', long = "type", help = "Entry type", num_args = 1..)]
    #[clap(value_enum)]
    types: Vec<Type>,
}

fn convert_entry_file_type(entry: DirEntry) -> Type {
    if entry.file_type().is_file() {
        Type::File
    } else if entry.file_type().is_dir() {
        Type::Dir
    } else if entry.file_type().is_symlink() {
        Type::Link
    } else {
        unreachable!()
    }
}

pub fn get_args() -> MyResult<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> MyResult<()> {
    for path in args.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if !args.names.is_empty() {
                        let name = entry.file_name().to_string_lossy();
                        if !args.names.iter().any(|re| re.is_match(&name)) {
                            continue;
                        }
                    }

                    if !args.types.is_empty() {
                        let entry_type = convert_entry_file_type(entry.clone());
                        if !args.types.contains(&entry_type) {
                            continue;
                        }
                    }

                    println!("{}", entry.path().display());
                }
            }
        }
    }

    Ok(())
}
