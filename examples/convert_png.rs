use clap::{App, Arg};
use std::io::prelude::*;
use std::fs;
use image;
use icns::encode::Encoder;

fn main() {
    let matches = App::new("convert png to icns")
        .version("0.1.0")
        .author("Jack Mordaunt <jackmordaunt@gmail.com>")
        .about("easily convert png to icns")
        .arg(Arg::with_name("input")
            .required(true)
            .takes_value(true)
            .short("i")
            .help("path to input png image"))
        .arg(Arg::with_name("output")
            .required(true)
            .takes_value(true)
            .short("o")
            .help("path to output icns image"))
        .get_matches();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let mut png: Vec<u8> = vec![];
    fs::File::open(&input)
        .expect("opening input file")
        .read_to_end(&mut png)
        .expect("buffering input file");
    let png = image::load_from_memory(&png)
        .expect("decoding png from buffer");
    let mut output = fs::File::create(&output)
        .expect("creating output file");
    Encoder::new(&mut output)
        .encode(&png)
        .expect("encoding icns");
}