use pest::Parser;
use pest_derive::Parser;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "slick", about = "The Slick Compiler usage.")]
struct Opt {
    /// Input file
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    file: PathBuf,
}

#[derive(Parser)]
#[grammar = "src/parser.pest"]
struct BDParser;

fn main() {
    let opt = Opt::from_args();

    let s = std::fs::read_to_string(opt.file).unwrap();
    let successful_parse = BDParser::parse(Rule::FILE, &s).unwrap();
    println!("{:#?}", successful_parse)
}
