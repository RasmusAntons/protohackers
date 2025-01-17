pub mod t00_smoke_test;
use clap::Parser;

#[derive(Parser)]
struct Args {
    task: u8,
    #[arg(long, default_value="release")]
    profile: String
}

fn main() {
    let args = Args::parse();
    match args.task {
        1 => {
            let _ = t00_smoke_test::main();
        }
        _ => {
            println!("Unknown task");
        }
    }
}
