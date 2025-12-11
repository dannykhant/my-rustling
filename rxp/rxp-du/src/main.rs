use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Cli::parse();
    println!("Hello, {}!", args.name);
}
