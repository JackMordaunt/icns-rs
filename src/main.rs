mod decode;
mod encode;
mod os_type;

use clap::{App, Arg};
use decode::Decoder;
use encode::Encoder;
use image::{self, png::PNGEncoder, ColorType, ImageFormat};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Cursor};

fn main() {
    let cli = App::new("icnsify")
        .version("0.2.0")
        .author("Jack Mordaunt <jackmordaunt@gmail.com>")
        .about("easily create icns icons from png images")
        .arg(
            Arg::with_name("in")
                .short("i")
                .long("input")
                .takes_value(true)
                .requires("out")
                .help("path to input file"),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("output")
                .takes_value(true)
                .requires("in")
                .help("path to output file"),
        )
        .arg(
            Arg::with_name("decode")
                .long("decode")
                .help("decode an icns into a png (reverse the direction)"),
        )
        .get_matches();
    if let (Some(src), Some(out)) = (cli.value_of("in"), cli.value_of("out")) {
        if cli.is_present("decode") {
            let src = BufReader::new(File::open(&src).expect("opening src file"));
            let img = Decoder::new(src).decode().expect("decoding png from icns");
            img.save(&out).expect("writing png");
        } else {
            let src = image::open(&src).expect("decoding input image");
            let out = BufWriter::new(File::create(&out).expect("creating output file"));
            Encoder::new(out)
                .encode(&src.to_rgba())
                .expect("encoding icns");
        }
    } else {
        // BUG: Piping doesn't work with pwsh version of cat (builtin, I think).
        // What is the powershell way of piping?
        // Check on Unix.
        if cli.is_present("decode") {
            let img = Decoder::new(BufReader::new(io::stdin().lock()))
                .decode()
                .expect("decoding icns")
                .into_rgba();
            PNGEncoder::new(io::stdout().lock())
                .encode(&img, img.width(), img.height(), ColorType::Rgb8)
                .expect("encoding png");
        } else {
            let mut buffer: Vec<u8> = vec![];
            io::copy(&mut io::stdin().lock(), &mut buffer).expect("reading from stdin");
            let img = image::load(&mut Cursor::new(buffer), ImageFormat::Png).expect("loading png");
            Encoder::new(BufWriter::new(io::stdout().lock()))
                .encode(&img.to_rgba())
                .expect("encoding icns");
        }
    }
}
