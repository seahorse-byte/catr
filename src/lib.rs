use std::error::Error;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Your Name")
        .about("A simple cat command")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Files to concatenate")
                .multiple(true)
                .required(true)
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number all output lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number nonempty output lines"),
        )
        .get_matches();

        Ok(Config {
            files: matches.values_of_lossy("files").unwrap(),
            number_lines: matches.is_present("number_lines"),
            number_nonblank_lines: matches.is_present("number_nonblank_lines")
        })

}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
