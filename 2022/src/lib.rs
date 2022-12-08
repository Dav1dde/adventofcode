pub mod cli;
pub mod grid;

pub type Input = std::io::BufReader<Box<dyn std::io::Read>>;
