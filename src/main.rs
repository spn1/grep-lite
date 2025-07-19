use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::prelude::*;
use clap::ArgAction;
use regex::Regex;
use clap::{Command, Arg, builder};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
        for _line in reader.lines() {
        let line = _line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns in text")    
        .arg(
            Arg::new("pattern")
                .short('p')
                .help("The pattern to search for")
                .required(true)
                .action(ArgAction::Set)
                .value_parser(builder::NonEmptyStringValueParser::new())
        ).arg(
            Arg::new("input")
                .short('i')
                .help("File to search")
                .required(true)
                .action(ArgAction::Set)
                .value_parser(builder::NonEmptyStringValueParser::new())
        )
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern.as_str()).unwrap();

    let input = args.get_one::<String>("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    process_lines(reader, re);
}
