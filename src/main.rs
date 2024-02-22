use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use qbe::{Function, Linkage, Module, Type, Value};
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
    let successful_parse = BDParser::parse(Rule::FILE, &s);
    // println!("{:?}", successful_parse);
    let successful_parse = successful_parse.unwrap();

    let mut module = Module::new();
    generate(successful_parse, &mut module);
    // println!("{:#?}", successful_parse)
    println!("{}", module)
}

fn generate(parsed: Pairs<'_, Rule>, module: &mut Module) {
    for i in parsed {
        match i.as_rule() {
            Rule::EOI => return,
            Rule::FN_DEF => generate_function(i, module),
            _ => {}
        }
    }
}

fn generate_function(pair: Pair<'_, Rule>, module: &mut Module) {
    let mut func = pair.clone().into_inner();
    let func_name = func.next().unwrap().as_span().as_str().to_string();
    let mut function;
    // println!("{:#?}", func);
    match func.next() {
        Some(args) => match args.as_rule() {
            Rule::FN_ARGS => {
                let func_args = args.into_inner();
                // println!("{:?}", func_args);
                let func_args: Vec<&str> = func_args
                    .into_iter()
                    .map(|x| x.as_span().as_str())
                    .collect();
                let func_args: Vec<(&str, &str)> =
                    func_args.chunks(2).map(|x| (x[0], x[1])).collect();
                // println!("{:#?}", func_args);
                function = Function::new(
                    Linkage::public(),
                    func_name,
                    func_args
                        .into_iter()
                        .map(|(x, y)| match y {
                            ":int" => (Type::Word, Value::Temporary(x.into())),
                            ":float" => (Type::Single, Value::Temporary(x.into())),
                            ":bool" => (Type::Halfword, Value::Temporary(x.into())),
                            ":uint" => (Type::Word, Value::Temporary(x.into())),
                            &_ => (Type::Word, Value::Temporary(x.into())),
                        })
                        .collect(),
                    {
                        if let Some(a) = func.next() {
                            match a.as_str() {
                                ":int" => Some(Type::Word),
                                ":float" => Some(Type::Single),
                                ":bool" => Some(Type::Halfword),
                                ":uint" => Some(Type::Word),
                                &_ => Some(Type::Word),
                            }
                        } else {
                            None
                        }
                    },
                );
            }
            Rule::BODY => {
                function =
                    Function::new(Linkage::public(), func_name, Vec::new(), Some(Type::Word));
            }
            _ => return,
        },
        None => return,
    };

    function.add_block("start".into());

    module.add_function(function);
}
