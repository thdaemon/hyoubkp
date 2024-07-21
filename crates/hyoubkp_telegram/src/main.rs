use std::io::{BufReader, BufRead, Write};

use bc_core::Context;

use anyhow::Result;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
struct Command {
    /// Configuration file, default is `config.yaml' in cwd
    #[clap(short = 'c', long)]
    config: Option<String>,

    /// Input file that contains the expressions, NL-splited. default stdin
    #[clap(short = 'i', long)]
    input: Option<String>,

    /// Output file, default stdout
    #[clap(short = 'o', long)]
    output: Option<String>,

    /// Export format
    #[clap(short = 'f', long)]
    format: Option<String>,

}

fn main() -> Result<()> {
    let args = <Command as clap::Parser>::parse();

    let mut context = Context::new_with_config_path("examples/chn-1.yaml")?;

    let input: Box<dyn BufRead> = match args.input {
        Some(filename) => Box::new(BufReader::new(std::fs::File::open(filename)?)),
        None => Box::new(BufReader::new(std::io::stdin())),
    };

    let mut output: Box<dyn Write> = match args.output {
        Some(filename) => Box::new(std::fs::File::create(filename)?),
        None => Box::new(std::io::stdout()),
    };

    for expr in input.lines() {
        match context.parse_expr(expr?) {
            Ok(trans) => output.write_all(format!("{}", trans).as_bytes())?,
            Err(s) => output.write_all(format!("Expr parse error: {}", s).as_bytes())?,
        }
    }

    //println!("{:#?}", context);

    Ok(())
}
