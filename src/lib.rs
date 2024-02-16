use clap::{Parser, ValueEnum};
use regex::Regex;

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
    #[arg(value_name = "NAME", short, long = "name", help = "Name")]
    names: Vec<Regex>,
    #[arg(
        value_name = "TYPE",
        short = 't',
        long = "type",
        help = "Entry type",
        value_enum
    )]
    types: Vec<Type>,
}

pub fn get_args() -> MyResult<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> MyResult<()> {
    println!("{:?}", args);
    Ok(())
}
