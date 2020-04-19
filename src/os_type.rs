use std::str::FromStr;

/// OSType is an enum of various icon types that can exist inside an icns
/// container. This enum only contains the high resolution variants that we care
/// about. Find the full list here: https://en.wikipedia.org/wiki/Apple_Icon_Image_format
#[derive(Clone)]
pub enum OSType {
    IC10,
    IC14,
    IC13,
    IC07,
    IC12,
    IC11,
}

impl OSType {
    /// Returns the largest OSType for the given dimension.
    pub fn nearest(d: u32) -> Self {
        for variant in OSType::variants() {
            if d >= variant.size() {
                return variant;
            }
        }
        OSType::IC11
    }
    /// Get a list of all OSType variants.
    pub fn variants() -> Vec<OSType> {
        vec![
            OSType::IC10,
            OSType::IC14,
            OSType::IC13,
            OSType::IC07,
            OSType::IC12,
            OSType::IC11,
        ]
    }
    /// Size in pixels.
    pub fn size(&self) -> u32 {
        match self {
            OSType::IC10 => 1024,
            OSType::IC14 => 512,
            OSType::IC13 => 256,
            OSType::IC07 => 128,
            OSType::IC12 => 64,
            OSType::IC11 => 32,
        }
    }
    /// 4 byte header corresponding to the OSType.
    pub fn header(&self) -> &'static str {
        match self {
            OSType::IC10 => "ic10",
            OSType::IC14 => "ic14",
            OSType::IC13 => "ic13",
            OSType::IC07 => "ic07",
            OSType::IC12 => "ic12",
            OSType::IC11 => "ic11",
        }
    }
    /// Get a list of all variants equal to or smaller than the current one.
    pub fn smaller_variants(&self) -> Vec<OSType> {
        let variants = OSType::variants();
        for (ii, v) in variants.iter().enumerate() {
            if v.size() <= self.size() {
                return variants[ii..].to_vec();
            }
        }
        variants
    }
}

impl FromStr for OSType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ic10" => Ok(OSType::IC10),
            "ic14" => Ok(OSType::IC14),
            "ic13" => Ok(OSType::IC13),
            "ic07" => Ok(OSType::IC07),
            "ic12" => Ok(OSType::IC12),
            "ic11" => Ok(OSType::IC11),
            _ => Err(format!("{} is not an icns OSType", s)),
        }
    }
}
