use crate::os_type::OSType;
use byteorder::{BigEndian, ReadBytesExt};
use image::{self, DynamicImage};
use std::error::Error;
use std::io::{self, Read};

/// Decoder decodes the largest image from an icns image.
pub struct Decoder<R: Read> {
    r: R,
}

impl<R: Read> Decoder<R> {
    /// Create a new Decoder that reads from `r`.
    pub fn new(r: R) -> Self {
        Decoder { r }
    }
    /// Decode icns from reader into image buffer.
    /// Picks the largest image in the icns container.
    ///
    // This algorithm simply reads off the byte stream `r`, expecting
    // data to come in a certain order.
    //
    // If the data is not in the expected order, we return an error.
    // ICNS is a container format that generally contains differently sized
    // png images, and some other bits and pieces like a table of contents.
    //
    // Each image is preceeded by an 8 byte header containing 2 data points:
    // 1. 4 byte ascii ID indicating the OS_Type.
    // 2. 4 byte unsigned integer (u32) that contains the size of the png
    //  content including the header.
    //
    // Once we've parsed out all the icon data, we return the largest available
    // image.
    pub fn decode(&mut self) -> Result<DynamicImage, Box<dyn Error>> {
        // Note(jfm): 4 bytes is the width of the headers and the size integers,
        // hence this buffer is 4 bytes long.
        let mut buffer: [u8; 4] = [0; 4];
        self.r.read_exact(&mut buffer)?;

        // Check the header.
        let header = std::str::from_utf8(&buffer)
            .map_err(|e| format!("parsing header as utf8 string: {:?}", e))?;
        if header != "icns" {
            return Err(format!("invalid header for icns file: got {}", header).into());
        }

        let _size = self.r.read_u32::<BigEndian>()?;
        let mut icons: Vec<IconReader> = Vec::new();

        loop {
            if let Err(err) = self.r.read_exact(&mut buffer) {
                if err.kind() == io::ErrorKind::UnexpectedEof {
                    break;
                } else {
                    return Err(err.into());
                }
            };
            match std::str::from_utf8(&buffer)
                .map_err(|e| format!("parsing chunk as utf8 string: {:?}", e))
            {
                Ok("TOC ") => {
                    // Note(jfm): Advance the reader to skip over the TOC.
                    // TODO(jfm): Could we use TOC to jump to the PNG we care about?
                    let toc_size = self.r.read_u32::<BigEndian>()?;
                    self.r
                        .read_exact(&mut Vec::with_capacity(toc_size as usize))?;
                    continue;
                }
                Ok("icnV") => continue,
                Ok(next) => {
                    if let Ok(os_type) = next.parse::<OSType>() {
                        let data_size = self.r.read_u32::<BigEndian>()?;
                        if data_size == 0 {
                            continue;
                        }
                        let mut data: Vec<u8> = vec![0; (data_size as usize) - 8];
                        self.r
                            .read_exact(&mut data)
                            .map_err(|e| format!("reading into data buffer: {}", e))?;
                        assert!(
                            data.len() > 0,
                            "data buffer should not be empty after reading"
                        );
                        icons.push(IconReader { os_type, data });
                    }
                }
                Err(_) => continue,
            };
        }

        if icons.is_empty() {
            return Err("no icons found".into());
        }

        let largest = icons
            .into_iter()
            .fold(None, |mut largest: Option<IconReader>, next| {
                if let Some(l) = largest.as_ref() {
                    if next.os_type.size() > l.os_type.size() {
                        largest.replace(next);
                    }
                } else {
                    largest = Some(next);
                }
                largest
            });
        if let Some(icon) = largest {
            Ok(image::load_from_memory(&icon.data.as_slice())
                .map_err(|e| format!("loading image from memory: {}", e))?)
        } else {
            Err("no icons found".into())
        }
    }
}

// IconReader assosciates os_type with icon data.
struct IconReader {
    os_type: OSType,
    data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Encoder;
    use std::error::Error;

    #[test]
    fn codec_symmetry() -> Result<(), Box<dyn Error>> {
        let input_img = image::RgbaImage::new(64, 64);
        let mut icns_buffer: Vec<u8> = vec![];
        Encoder::new(&mut icns_buffer)
            .encode(&input_img)
            .map_err(|e| format!("encoding icns: {}", e))?;
        let got_img = Decoder::new(icns_buffer.as_slice())
            .decode()
            .map_err(|e| format!("decoding icns: {}", e))?
            .into_rgba();
        let (l, r) = (input_img.dimensions(), got_img.dimensions());
        assert_eq!(l, r, "dimension mismatch: {:?} != {:?}", l, r);
        assert_eq!(input_img.into_vec(), got_img.into_vec(), "buffer mismatch");
        Ok(())
    }
}
