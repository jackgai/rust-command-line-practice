use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Yizhi Gai <XX@XX.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("lines")
            .short("n")
            .long("lines")
            .value_name("LINES")
            .help("Number of lines")
            .default_value("10")
            .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("bytes")
            .short("c")
            .long("bytes")
            .value_name("BYTES")
            .help("Number of bytes")
            .takes_value(true),
        )
        .arg(
            Arg::with_name("files")
            .value_name("FILES")
            .help("Input file(s)")
            .multiple(true)
            .default_value("-"),
        )
        .get_matches();

    let bytes = 10;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: 10,
        bytes: Some(bytes),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n>0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an Ok integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),3);
    // Any string is a error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(),"foo".to_string());
    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(),"0".to_string());
}
