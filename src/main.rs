mod encode;
mod os_type;

use std::io::{self, BufReader, BufWriter};
use std::fs::File;
use clap::{App, Arg};
use image;
use icns::Encoder;

fn main() {
    let cli = App::new("icnsify")
        .version("0.1.0")
        .author("Jack Mordaunt <jackmordaunt@gmail.com>")
        .about("easily create icns icons from png images")
        .arg(Arg::with_name("in")
            .short("i")
            .long("input")
            .takes_value(true)
            .requires("out")
            .help("path to input file"))
        .arg(Arg::with_name("out")
            .short("o")
            .long("output")
            .takes_value(true)
            .requires("in")
            .help("path to output file"))
        .get_matches();
    if let (Some(src), Some(out)) = (cli.value_of("in"), cli.value_of("out")) {
        let src = image::open(&src)
            .expect("decoding input image");
        let out = BufWriter::new(File::create(&out)
            .expect("creating output file"));
        Encoder::new(out).encode(&src.to_rgba())
            .expect("encoding icns");
    } else {
        let mut buf: Vec<u8> = vec![];
        let stdin = io::stdin();
        let mut stdin = BufReader::new(stdin.lock());
        io::copy(&mut stdin, &mut buf)
            .expect("reading from stdin");
        let src = image::load_from_memory(&buf)
            .expect("decoding input image");
        Encoder::new(BufWriter::new(io::stdout().lock()))
            .encode(&src.to_rgba())
            .expect("encoding icns");
    }
}