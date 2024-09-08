use std::io::{BufRead, BufReader, Write};

use anyhow::Result;
use hyoubkp::{
    datagen::{DataGenDispatch, DataGenKind},
    tokmap::TokenMapperKind,
};
use hyoubkp_base::datagen::DataGen;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
struct Command {
    /// token mapper type
    #[clap(short = 't', long)]
    token_mapper: TokenMapperKind,

    /// Input file that contains the expressions, NL-splited. default stdin
    #[clap(short = 'i', long)]
    input: Option<String>,

    /// Output file, default stdout
    #[clap(short = 'o', long)]
    output: Option<String>,

    /// Data-gen backend
    #[clap(short = 'd', long)]
    datagen: DataGenKind,

    /// Data-gen backend options (Not implemented)
    #[clap(short = 'D', long)]
    datagen_options: Vec<String>,

    /// Do not quit while expression parsing failed (syntax error).
    /// And then place a fallback/placeholder entry.
    /// (Not implemented)
    #[clap(short = 'C', long, default_value_t = false)]
    continue_on_syntax_error: bool,

    /// Error-quit on any ambiguities occured while transaction constructing,
    /// Insteads placing some fallback/placeholder entries.
    /// (Not implemented)
    #[clap(short = 'A', long, default_value_t = false)]
    treat_ambiguity_as_error: bool,
}

fn main() -> Result<()> {
    let args = <Command as clap::Parser>::parse();

    let mut executor = hyoubkp::executor::Executor::new(args.token_mapper);
    let datagen_impl = DataGenDispatch::new(args.datagen);

    let input: Box<dyn BufRead> = match args.input {
        Some(filename) => Box::new(BufReader::new(std::fs::File::open(filename)?)),
        None => Box::new(BufReader::new(std::io::stdin())),
    };

    let mut output: Box<dyn Write> = match args.output {
        Some(filename) => Box::new(std::fs::File::create(filename)?),
        None => Box::new(std::io::stdout()),
    };

    let mut number = 0;
    for expr in input.lines() {
        let expr = expr?;

        if expr.is_empty() {
            continue;
        } else if expr.starts_with('.') {
            executor.parse_directive(expr)?;
        } else {
            let trans = executor.parse_expr(expr)?;
            datagen_impl.write_to(&mut output, std::slice::from_ref(&trans), number)?;
            number += 1;
        }
    }

    Ok(())
}
