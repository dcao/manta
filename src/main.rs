use std::{env, fs};

mod parse;
mod repr;

const VERSION: &'static str = "manta 0.1.0";

const USAGE: &'static str = r"\
manta 0.1.0
David Cao <david@cao.sh>
efficient array programming

USAGE:
    mt
    mt <FILE>
    mt -e <EXPR>
    mt -h
    mt -V
";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        // TODO: repl mode
        // rustyline or linefeed
        todo!();
    } else if &args[1] == "-V" {
        println!("{}", VERSION);
    } else if &args[1] == "-e" {
        if let Some(i) = args.get(2) {
            let mut ctx = parse::Context::new();
            let p = parse::parser::ExprParser::new().parse(&mut ctx, parse::tokens(i));

            dbg!(p);
        } else {
            panic!("no expr specified");
        }
    } else if &args[1] == "-h" {
        println!("{}", USAGE);
    } else {
        let contents = fs::read_to_string(&args[1]).expect("invalid file");

        println!("text:\n{}", contents);
    }
}
