use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/parser.pest"]
struct BDParser;

fn main() {
    let successful_parse = BDParser::parse(Rule::file, "let a");
    println!("{:?}", successful_parse)
}
