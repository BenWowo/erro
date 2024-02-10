use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser, Debug)]
struct Args {
    // path of the input file
    #[arg()]
    file_path: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // println!("Hello {}!", args.file_path);
    let text: String = read_to_string(args.file_path)?;
    println!("{}", text);

    Ok(())
}
