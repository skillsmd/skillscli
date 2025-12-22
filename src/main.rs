use clap::Parser;

#[derive(Parser)]
#[command(name = "skills")]
#[command(about = "A Rust CLI application", long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Verbose mode enabled");
    }

    println!("Welcome to skills!");

    Ok(())
}
