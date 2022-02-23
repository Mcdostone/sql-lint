use clap::Parser;
use sql_lint::format;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(parse(from_os_str), short, long)]
    input: Option<PathBuf>,
    #[clap(parse(from_os_str), short, long)]
    output: Option<PathBuf>,
    #[clap()]
    query: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let contents = match args.input {
        Some(ref i) => fs::read_to_string(i).unwrap(),
        None => match args.query.len() {
            0 => {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                buffer
            }
            _ => args.query.join(""),
        },
    };

    match format(&contents) {
        Ok(formatted) => write_output(&args, &formatted),
        Err(e) => {
            eprintln!("{}", e);
            write_output(&args, &contents)
        }
    }
}

fn write_output(args: &Args, content: &str) -> Result<(), Box<dyn Error>> {
    match &args.output {
        Some(o) => std::fs::write(o, content).map_err(|e| Box::new(e) as Box<dyn Error>),
        None => {
            println!("{}", content);
            Ok(())
        }
    }
}
