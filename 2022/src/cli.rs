use crate::Input;
use anyhow::Context;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
struct Opts {
    #[arg(short, long)]
    time: bool,

    #[arg(short = '1', long, group = "day")]
    part1: bool,
    #[arg(short = '2', long, group = "day")]
    part2: bool,

    input: Option<PathBuf>,
}

pub fn run<P1, P2, R>(part1: P1, part2: P2) -> anyhow::Result<R>
where
    P1: FnOnce(Input) -> anyhow::Result<R>,
    P2: FnOnce(Input) -> anyhow::Result<R>,
    R: std::fmt::Display,
{
    let opts = Opts::parse();

    let input: Input = if let Some(input) = opts.input {
        let file = File::open(&input).with_context(|| format!("Failed to read file {input:?}"))?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(BufReader::new(std::io::stdin()))
    };

    let begin = Instant::now();

    let result = if opts.part2 {
        part2(input)
    } else {
        part1(input)
    };

    if opts.time {
        eprintln!(
            "[Execution time]: {:?}",
            Instant::now().duration_since(begin)
        );
    }

    match result {
        Ok(result) => {
            println!("{result}");
            Ok(result)
        }
        Err(err) => {
            eprintln!("Execution failed!");
            eprintln!("===============================");
            eprintln!("{err:?}");
            eprintln!("===============================");
            Err(err)
        }
    }
}
