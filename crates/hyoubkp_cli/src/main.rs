use std::io::{BufRead, BufReader, Write};

use anyhow::Result;
use clap::ValueEnum;
use hyoubkp::{
    datagen::{DataGenDispatch, DataGenKind},
    tokmap::TokenMapperKind,
};
use hyoubkp_base::{datagen::DataGen, tokmap::TokenMapperOption};

#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
struct Command {
    /// token mapper type
    #[clap(short = 't', long, default_value = "example")]
    token_mapper: TokenMapperKind,

    /// token mapper options
    #[arg(short = 'T', long, value_parser = parse_key_value::<TokenMapperOption>)]
    token_mapper_options: Vec<(TokenMapperOption, String)>,

    /// print all available token mapper options
    #[clap(long)]
    print_token_mapper_options: bool,

    /// Input file that contains the expressions, NL-splited. default stdin
    #[clap(short = 'i', long)]
    input: Option<String>,

    /// Output file, default stdout
    #[clap(short = 'o', long)]
    output: Option<String>,

    /// Data-gen backend
    #[clap(short = 'd', long, default_value = "str")]
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

    if args.print_token_mapper_options {
        for opt in TokenMapperOption::OPTIONS.iter() {
            println!(
                "-T {} [supported by: {}]",
                opt.description(),
                TokenMapperKind::generate_option_supported_tokmap_names(*opt).join(", ")
            )
        }

        return Ok(());
    }

    let tokmap_options = args.token_mapper_options.into_iter().collect();

    let mut executor = hyoubkp::executor::Executor::new(args.token_mapper, &tokmap_options)?;
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

fn parse_key_value<T: ValueEnum>(s: &str) -> Result<(T, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err("Invalid format, expected key=value".to_string());
    }

    let key = T::from_str(parts[0], true)?;

    let value = parts[1].to_string();
    Ok((key, value))
}
