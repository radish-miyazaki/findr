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

pub fn get_args() -> MyResult<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> MyResult<()> {
    let type_filter = |entry: &DirEntry| {
        args.types.is_empty()
            || args.types.iter().any(|t| match t {
                Type::Link => entry.file_type().is_symlink(),
                Type::Dir => entry.file_type().is_dir(),
                Type::File => entry.file_type().is_file(),
            })
    };

    let name_filter = |entry: &DirEntry| {
        args.names.is_empty()
            || args
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in args.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Ok(entry) => Some(entry),
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|e| e.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }

    Ok(())
}
