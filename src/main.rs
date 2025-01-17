mod t00_smoke_test;
mod t01_prime_time;

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
        0 => {
            let _ = t00_smoke_test::main();
        }
        1 => {
            let _ = t01_prime_time::main();
        }
        _ => {
            println!("Unknown task");
        }
    }
}
