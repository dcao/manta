use std::{env, fs};

mod parse;

const VERSION: &'static str = "manta 0.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        // TODO: repl mode
        // rustyline or linefeed
        todo!();
    } else if &args[1] == "-V" {
        println!("{}", VERSION);
    } else {
        let contents = fs::read_to_string(&args[1])
            .expect("!file");

        println!("text:\n{}", contents);
    }
}
