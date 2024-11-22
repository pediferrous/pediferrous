use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Config {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    GenTestFile,
}

fn main() {
    match Config::parse().cmd {
        Cmd::GenTestFile => pediferrous::gen_test_file(),
    }
}
