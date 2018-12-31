use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};
use clap::{App, Arg};
use image;
use icns::Encoder;

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

    // Load inputs. Since we specified "required", these unwraps wont fail. 
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    // Read the png file into a buffer.
    let mut png: Vec<u8> = vec![];
    BufReader::new(File::open(&input)
        .expect("opening input file"))
        .read_to_end(&mut png)
        .expect("buffering input file");
    
    // Load a DynamicImage object from the raw png data. 
    let png = image::load_from_memory(&png)
        .expect("decoding png from buffer");
    
    // Create the output file. 
    let mut output = BufWriter::new(File::create(&output)
        .expect("creating output file"));

    // Encode the png as icns into the output file. 
    // Note we use to_rgba to convert the DynamicImage into an RgbaImage. 
    Encoder::new(&mut output)
        .encode(&png.to_rgba())
        .expect("encoding icns");
}