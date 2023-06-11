use clap::Parser;
use std::fs;
// use std::path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    target_dir_path: String,

    /// Number of times to greet
    #[arg(short, long)]
    file_name: String,
}

fn main() {
    let args = Args::parse();

    let dir = fs::read_dir(&args.target_dir_path).unwrap();

    for file in dir.into_iter() {
        let file_name = file.unwrap().file_name();
        println!("{:?}", file_name);
    }
}
