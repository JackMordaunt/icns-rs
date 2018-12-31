mod encode;
mod os_type;

use std::io::BufWriter;
use std::fs::File;
use clap::{App, Arg};
use image;
use icns::Encoder;

fn main() {
    let cli = App::new("icnsify")
        .version("0.1.0")
        .author("Jack Mordaunt <jackmordaunt@gmail.com>")
        .about("easily create icns icons from png images")
        .arg(Arg::with_name("input")
            .short("i")
            .takes_value(true)
            .requires("output")
            .help("path to input file"))
        .arg(Arg::with_name("output")
            .short("o")
            .takes_value(true)
            .requires("input")
            .help("path to output file"))
        .get_matches();
    if let (Some(src), Some(out)) = (cli.value_of("input"), cli.value_of("output")) {
        let src = image::open(&src)
            .expect("decoding input image");
        let out = BufWriter::new(File::create(&out)
            .expect("creating output file"));
        Encoder::new(out).encode(&src.to_rgba())
            .expect("encoding icns");
    }
}