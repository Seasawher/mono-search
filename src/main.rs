use clap::Parser;
use ignore::Walk;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // 探索対象のディレクトリへのパス
    #[arg(long)]
    haystack: String,

    // 探したいファイルの名前
    #[arg(long)]
    needle: String,
}

fn main() {
    let args = Args::parse();

    for result in Walk::new(&args.haystack) {
        match result {
            Ok(entry) => {
                if entry.file_name().to_str().unwrap() == args.needle {
                    println!("{}", entry.path().display());
                }
            },
            Err(error) => println!("{}", error),
        }
    }
}
