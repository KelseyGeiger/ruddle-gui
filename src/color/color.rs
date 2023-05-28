pub use super::colorformat::*;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Gray8(Gray8),
    Gray16(Gray16),
    Grayf(GrayF),
    RGB(Rgb),
    SRGB(Srgb),
    RGB48(Rgb48),
    RGBA(Rgba),
    RGBA64(Rgba64),
    RGBf(RgbF),
    SRGBf(SrgbF),
    RGBAf(RgbaF),
    HSV(Hsv),
    HSL(Hsl),
    CIEXYZ(CieXyz),
    CIELab(CieLab)
}

impl Default for Color {
    fn default() -> Color {
        let rgb = Rgb::default();
        Color::RGB(rgb)
    }
}

#[allow(dead_code)]
impl Color {
    pub fn get_format(&self) -> ColorFormatName {
        match *self {
            Color::Gray8(_g8) => {
                Gray8::format_name()
            },
            Color::Gray16(_g16) => {
                Gray16::format_name()
            },
            Color::Grayf(_gf) => {
                GrayF::format_name()
            },
            Color::RGB(_rgb) => {
                Rgb::format_name()
            },
            Color::SRGB(_srgb) => {
                Srgb::format_name()
            },
            Color::RGB48(_rgb_w) => {
                Rgb48::format_name()
            },
            Color::RGBA(_rgba) => {
                Rgba::format_name()
            },
            Color::RGBA64(_rgba_w) => {
                Rgba64::format_name()
            },
            Color::RGBf(_rgbf) => {
                RgbF::format_name()
            },
            Color::SRGBf(_srgbf) => {
                SrgbF::format_name()
            },
            Color::RGBAf(_rgbaf) => {
                RgbaF::format_name()
            },
            Color::HSV(_hsv) => {
                Hsv::format_name()
            },
            Color::HSL(_hsl) => {
                Hsl::format_name()
            },
            Color::CIEXYZ(_xyz) => {
                CieXyz::format_name()
            },
            Color::CIELab(_lab) => {
                CieLab::format_name()
            }
        }
    }

    pub fn get_bytes_per_pixel(&self) -> usize {
        match *self {
            Color::Gray8(_g8) => {
                Gray8::bytes_per_pixel()
            },
            Color::Gray16(_g16) => {
                Gray16::bytes_per_pixel()
            },
            Color::Grayf(_gf) => {
                GrayF::bytes_per_pixel()
            },
            Color::RGB(_rgb) => {
                Rgb::bytes_per_pixel()
            },
            Color::SRGB(_srgb) => {
                Srgb::bytes_per_pixel()
            },
            Color::RGB48(_rgb_w) => {
                Rgb48::bytes_per_pixel()
            },
            Color::RGBA(_rgba) => {
                Rgba::bytes_per_pixel()
            },
            Color::RGBA64(_rgba_w) => {
                Rgba64::bytes_per_pixel()
            },
            Color::RGBf(_rgbf) => {
                RgbF::bytes_per_pixel()
            },
            Color::SRGBf(_srgbf) => {
                SrgbF::bytes_per_pixel()
            },
            Color::RGBAf(_rgbaf) => {
                RgbaF::bytes_per_pixel()
            },
            Color::HSV(_hsv) => {
                Hsv::bytes_per_pixel()
            },
            Color::HSL(_hsl) => {
                Hsl::bytes_per_pixel()
            },
            Color::CIEXYZ(_xyz) => {
                CieXyz::bytes_per_pixel()
            },
            Color::CIELab(_lab) => {
                CieLab::bytes_per_pixel()
            }
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match *self {
            Color::Gray8(g8) => {
                g8.as_bytes()
            },
            Color::Gray16(g16) => {
                g16.as_bytes()
            },
            Color::Grayf(gf) => {
                gf.as_bytes()
            },
            Color::RGB(rgb) => {
                rgb.as_bytes()
            },
            Color::SRGB(srgb) => {
                srgb.as_bytes()
            },
            Color::RGB48(rgb_w) => {
                rgb_w.as_bytes()
            },
            Color::RGBA(rgba) => {
                rgba.as_bytes()
            },
            Color::RGBA64(rgba_w) => {
                rgba_w.as_bytes()
            },
            Color::RGBf(rgbf) => {
                rgbf.as_bytes()
            },
            Color::SRGBf(srgbf) => {
                srgbf.as_bytes()
            },
            Color::RGBAf(rgbaf) => {
                rgbaf.as_bytes()
            },
            Color::HSV(hsv) => {
                hsv.as_bytes()
            },
            Color::HSL(hsl) => {
                hsl.as_bytes()
            },
            Color::CIEXYZ(xyz) => {
                xyz.as_bytes()
            },
            Color::CIELab(lab) => {
                lab.as_bytes()
            }
        }
    }

    pub fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        match self {
            Color::Gray8(g8) => {
                g8.to_raw_parts()
            },
            Color::Gray16(g16) => {
                g16.to_raw_parts()
            },
            Color::Grayf(gf) => {
                gf.to_raw_parts()
            },
            Color::RGB(rgb) => {
                rgb.to_raw_parts()
            },
            Color::SRGB(srgb) => {
                srgb.to_raw_parts()
            },
            Color::RGB48(rgb_w) => {
                rgb_w.to_raw_parts()
            },
            Color::RGBA(rgba) => {
                rgba.to_raw_parts()
            },
            Color::RGBA64(rgba_w) => {
                rgba_w.to_raw_parts()
            },
            Color::RGBf(rgbf) => {
                rgbf.to_raw_parts()
            },
            Color::SRGBf(srgbf) => {
                srgbf.to_raw_parts()
            },
            Color::RGBAf(rgbaf) => {
                rgbaf.to_raw_parts()
            },
            Color::HSV(hsv) => {
                hsv.to_raw_parts()
            },
            Color::HSL(hsl) => {
                hsl.to_raw_parts()
            },
            Color::CIEXYZ(xyz) => {
                xyz.to_raw_parts()
            },
            Color::CIELab(lab) => {
                lab.to_raw_parts()
            },
        }
    }

    pub fn from_raw_parts(fmt: ColorFormatName, data: Vec<u8>) -> Result<Color, String> {
        match fmt {
            ColorFormatName::Gray8 => {
                let g8 = Gray8::from_bytes(&data);
                if g8.is_ok() {
                    Ok(Color::Gray8(g8.unwrap()))
                } else {
                    Err(g8.err().unwrap())
                }
            },
            ColorFormatName::Gray16 => {
                let g16 = Gray16::from_bytes(&data);
                if g16.is_ok() {
                    Ok(Color::Gray16(g16.unwrap()))
                } else {
                    Err(g16.err().unwrap())
                }
            },
            ColorFormatName::Grayf => {
                let gf = GrayF::from_bytes(&data);
                if gf.is_ok() {
                    Ok(Color::Grayf(gf.unwrap()))
                } else {
                    Err(gf.err().unwrap())
                }
            },
            ColorFormatName::RGB => {
                let rgb = Rgb::from_bytes(&data);
                if rgb.is_ok() {
                    Ok(Color::RGB(rgb.unwrap()))
                } else {
                    Err(rgb.err().unwrap())
                }
            },
            ColorFormatName::SRGB => {
                let srgb = Srgb::from_bytes(&data);
                if srgb.is_ok() {
                    Ok(Color::SRGB(srgb.unwrap()))
                } else {
                    Err(srgb.err().unwrap())
                }
            },
            ColorFormatName::RGB48 => {
                let rgb_w = Rgb48::from_bytes(&data);
                if rgb_w.is_ok() {
                    Ok(Color::RGB48(rgb_w.unwrap()))
                } else {
                    Err(rgb_w.err().unwrap())
                }
            },
            ColorFormatName::RGBA => {
                let rgba = Rgba::from_bytes(&data);
                if rgba.is_ok() {
                    Ok(Color::RGBA(rgba.unwrap()))
                } else {
                    Err(rgba.err().unwrap())
                }
            },
            ColorFormatName::RGBA64 => {
                let rgba_w = Rgba64::from_bytes(&data);
                if rgba_w.is_ok() {
                    Ok(Color::RGBA64(rgba_w.unwrap()))
                } else {
                    Err(rgba_w.err().unwrap())
                }
            },
            ColorFormatName::RGBf => {
                let rgbf = RgbF::from_bytes(&data);
                if rgbf.is_ok() {
                    Ok(Color::RGBf(rgbf.unwrap()))
                } else {
                    Err(rgbf.err().unwrap())
                }
            },
            ColorFormatName::SRGBf => {
                let srgbf = SrgbF::from_bytes(&data);
                if srgbf.is_ok() {
                    Ok(Color::SRGBf(srgbf.unwrap()))
                } else {
                    Err(srgbf.err().unwrap())
                }
            }
            ColorFormatName::RGBAf => {
                let rgbaf = RgbaF::from_bytes(&data);
                if rgbaf.is_ok() {
                    Ok(Color::RGBAf(rgbaf.unwrap()))
                } else {
                    Err(rgbaf.err().unwrap())
                }
            },
            ColorFormatName::HSV => {
                let hsv = Hsv::from_bytes(&data);
                if hsv.is_ok() {
                    Ok(Color::HSV(hsv.unwrap()))
                } else {
                    Err(hsv.err().unwrap())
                }
            },
            ColorFormatName::HSL => {
                let hsl = Hsl::from_bytes(&data);
                if hsl.is_ok() {
                    Ok(Color::HSL(hsl.unwrap()))
                } else {
                    Err(hsl.err().unwrap())
                }
            },
            ColorFormatName::CIEXYZ => {
                let xyz = CieXyz::from_bytes(&data);
                if xyz.is_ok() {
                    Ok(Color::CIEXYZ(xyz.unwrap()))
                } else {
                    Err(xyz.err().unwrap())
                }
            },
            ColorFormatName::CIELab => {
                let lab = CieLab::from_bytes(&data);
                if lab.is_ok() {
                    Ok(Color::CIELab(lab.unwrap()))
                } else {
                    Err(lab.err().unwrap())
                }
            },
        }
    }

    //This is an interative function if there is no direct converesion from one color format to another
    //Each step, if taken, converts to an intermediary format.
    //For example, HSV -> Lab will do HSV -> RGBf -> XYZ -> Lab
    pub fn convert(from: Color, to: ColorFormatName) -> Color {
        let mut conversion = from;
        loop {
            //Here be... not dragons, honestly. Just a lot of snakes. Like a *lot*. The cute kind not the dangerous kind.
            //(A lot of nested match statements that are individually very simple)
            match conversion {
                Color::Gray8(g8) => {
                    match to {
                        ColorFormatName::Gray8 => {
                            return conversion;
                        },
                        ColorFormatName::Gray16 => {
                            let g16: Gray16 = g8.into();
                            return Color::Gray16(g16);
                        },
                        ColorFormatName::Grayf => {
                            let gf: GrayF = g8.into();
                            return Color::Grayf(gf);
                        },
                        ColorFormatName::RGB => {
                            let rgb: Rgb = g8.into();
                            return Color::RGB(rgb);
                        },
                        ColorFormatName::RGBA => {
                            let rgba: Rgba = g8.into();
                            return Color::RGBA(rgba);
                        },
                        ColorFormatName::SRGB | ColorFormatName::RGB48 |
                        ColorFormatName::RGBf | ColorFormatName::SRGBf |
                        ColorFormatName::HSV | ColorFormatName::HSL |
                        ColorFormatName::CIEXYZ | ColorFormatName::CIELab =>
                        {
                            let rgb: Rgb = g8.into();
                            conversion = Color::RGB(rgb);
                            continue;
                        },
                        ColorFormatName::RGBA64 | ColorFormatName::RGBAf => {
                            let rgba: Rgba = g8.into();
                            conversion = Color::RGBA(rgba);
                            continue;
                        },
                    }
                },
                Color::Gray16(g16) => {
                    match to {
                        ColorFormatName::Gray16 => {
                            return conversion;
                        },
                        ColorFormatName::Gray8 => {
                            let g8: Gray8 = g16.into();
                            return Color::Gray8(g8);
                        },
                        ColorFormatName::Grayf => {
                            let gf: GrayF = g16.into();
                            return Color::Grayf(gf);
                        },
                        ColorFormatName::RGB => {
                            let rgb: Rgb = g16.into();
                            return Color::RGB(rgb);
                        },
                        ColorFormatName::RGB48 => {
                            let rgb_w: Rgb48 = g16.into();
                            return Color::RGB48(rgb_w);
                        },
                        ColorFormatName::RGBA64 => {
                            let rgba_w: Rgba64 = g16.into();
                            return Color::RGBA64(rgba_w);
                        },
                        ColorFormatName::RGBf => {
                            let rgbf: RgbF = g16.into();
                            return Color::RGBf(rgbf);
                        },
                        ColorFormatName::RGBAf => {
                            let rgbaf: RgbaF = g16.into();
                            return Color::RGBAf(rgbaf);
                        }
                        ColorFormatName::SRGB => {
                            let rgb: Rgb = g16.into();
                            let srgb: Srgb = rgb.into();
                            return Color::SRGB(srgb);
                        },
                        ColorFormatName::SRGBf => {
                            let rgb_f: RgbF = g16.into();
                            let srgbf: SrgbF = rgb_f.into();
                            return Color::SRGBf(srgbf);
                        },
                        ColorFormatName::RGBA => {
                            let rgbaf: RgbaF = g16.into();
                            let rgba: Rgba = rgbaf.into();
                            return Color::RGBA(rgba);
                        },
                        ColorFormatName::HSV | ColorFormatName::HSL |
                        ColorFormatName::CIEXYZ | ColorFormatName::CIELab =>
                        {
                            let rgbf: RgbF = g16.into();
                            conversion = Color::RGBf(rgbf);
                            continue;
                        },
                    }
                },
                Color::Grayf(gf) => {
                    match to {
                        ColorFormatName::Grayf => {
                            return conversion;
                        },
                        ColorFormatName::Gray8 => {
                            let g8: Gray8 = gf.into();
                            return Color::Gray8(g8);
                        },
                        ColorFormatName::Gray16 => {
                            let g16: Gray16 = gf.into();
                            return Color::Gray16(g16);
                        },
                        ColorFormatName::RGB => {
                            let rgb: Rgb = gf.into();
                            return Color::RGB(rgb);
                        },
                        ColorFormatName::SRGB => {
                            let srgb: Srgb = gf.into();
                            return Color::SRGB(srgb);
                        },
                        ColorFormatName::RGB48 => {
                            let rgb_w: Rgb48 = gf.into();
                            return Color::RGB48(rgb_w);
                        },
                        ColorFormatName::RGBA => {
                            let rgba: Rgba = gf.into();
                            return Color::RGBA(rgba);
                        },
                        ColorFormatName::RGBA64 => {
                            let rgba_w: Rgba64 = gf.into();
                            return Color::RGBA64(rgba_w);
                        },
                        ColorFormatName::RGBf => {
                            let rgbf: RgbF = gf.into();
                            return Color::RGBf(rgbf);
                        },
                        ColorFormatName::SRGBf => {
                            let srgbf: SrgbF = gf.into();
                            return Color::SRGBf(srgbf);
                        }
                        ColorFormatName::RGBAf => {
                            let rgbaf: RgbaF = gf.into();
                            return Color::RGBAf(rgbaf);
                        },
                        ColorFormatName::HSV | ColorFormatName::HSL |
                        ColorFormatName::CIEXYZ | ColorFormatName::CIELab => {
                            let rgbf: RgbF = gf.into();
                            conversion = Color::RGBf(rgbf);
                            continue;
                        },
                    }
                },
                Color::RGB(rgb) => {
                    match to {
                        ColorFormatName::RGB => {
                            return conversion;
                        },
                        ColorFormatName::Gray8 => {
                            let g8: Gray8 = rgb.into();
                            return Color::Gray8(g8);
                        },
                        ColorFormatName::Gray16 => {
                            let g16: Gray16 = rgb.into();
                            return Color::Gray16(g16);
                        },
                        ColorFormatName::Grayf => {
                            let gf: GrayF = rgb.into();
                            return Color::Grayf(gf);
                        },
                        ColorFormatName::SRGB => {
                            let srgb: Srgb = rgb.into();
                            return Color::SRGB(srgb);
                        },
                        ColorFormatName::RGB48 => {
                            let rgb_w: Rgb48 = rgb.into();
                            return Color::RGB48(rgb_w);
                        },
                        ColorFormatName::RGBA => {
                            let rgba: Rgba = rgb.into();
                            return Color::RGBA(rgba);
                        },
                        ColorFormatName::RGBA64 => {
                            let rgba_w: Rgba64 = rgb.into();
                            return Color::RGBA64(rgba_w);
                        },
                        ColorFormatName::RGBf => {
                            let rgbf: RgbF = rgb.into();
                            return Color::RGBf(rgbf);
                        },
                        ColorFormatName::SRGBf => {
                            let srgbf: SrgbF = rgb.into();
                            return Color::SRGBf(srgbf);
                        }
                        ColorFormatName::RGBAf => {
                            let rgbaf: RgbaF = rgb.into();
                            return Color::RGBAf(rgbaf);
                        },
                        ColorFormatName::HSV => {
                            let hsv: Hsv = rgb.into();
                            return Color::HSV(hsv);
                        },
                        ColorFormatName::HSL => {
                            let hsl: Hsl = rgb.into();
                            return Color::HSL(hsl);
                        },
                        ColorFormatName::CIEXYZ => {
                            let xyz: CieXyz = rgb.into();
                            return Color::CIEXYZ(xyz);
                        },
                        ColorFormatName::CIELab => {
                            let xyz: CieXyz = rgb.into();
                            let lab: CieLab = xyz.into();
                            return Color::CIELab(lab);
                        },
                    }
                },
                Color::SRGB(srgb) => {
                    match to {
                        ColorFormatName::SRGB => {
                            return conversion;
                        },
                        ColorFormatName::Grayf => {
                            let gf: GrayF = srgb.into();
                            return Color::Grayf(gf);
                        },
                        ColorFormatName::RGB => {
                            let rgb: Rgb = srgb.into();
                            return Color::RGB(rgb);
                        },
                        ColorFormatName::RGBf => {
                            let rgbf: RgbF = srgb.into();
                            return Color::RGBf(rgbf);
                        },
                        ColorFormatName::SRGBf => {
                            let srgbf: SrgbF = srgb.into();
                            return Color::SRGBf(srgbf);
                        }
                        ColorFormatName::RGBAf => {
                            let rgbf: RgbF = srgb.into();
                            let rgbaf: RgbaF = rgbf.into();
                            return Color::RGBAf(rgbaf);
                        },
                        ColorFormatName::HSV => {
                            let hsv: Hsv = srgb.into();
                            return Color::HSV(hsv);
                        },
                        ColorFormatName::HSL => {
                            let hsl: Hsl = srgb.into();
                            return Color::HSL(hsl);
                        },
                        ColorFormatName::CIEXYZ => {
                            let xyz: CieXyz = srgb.into();
                            return Color::CIEXYZ(xyz);
                        },
                        ColorFormatName::CIELab => {
                            let xyz: CieXyz = srgb.into();
                            let lab: CieLab = xyz.into();
                            return Color::CIELab(lab);
                        },
                        ColorFormatName::Gray8 | ColorFormatName::Gray16 => {
                            let gf: GrayF = srgb.into();
                            conversion = Color::Grayf(gf);
                            continue;
                        },
                        ColorFormatName::RGB48 | ColorFormatName::RGBA | ColorFormatName::RGBA64 => {
                            let rgb: Rgb = srgb.into();
                            conversion = Color::RGB(rgb);
                            continue;
                        },
                    }
                },
                Color::RGB48(rgb_w) => {
                    match to {
                        ColorFormatName::RGB48 => {
                            return conversion;
                        },
                        ColorFormatName::Gray16 => {
                            let g16: Gray16 = rgb_w.into();
                            return Color::Gray16(g16);
                        },
                        ColorFormatName::Grayf => {
                            let gf: GrayF = rgb_w.into();
                            return Color::Grayf(gf);
                        },
                        ColorFormatName::RGB => {
                            let rgb: Rgb = rgb_w.into();
                            return Color::RGB(rgb);
                        },
                        ColorFormatName::RGBf => {
                            let rgbf: RgbF = rgb_w.into();
                            return Color::RGBf(rgbf);
                        },
                        ColorFormatName::RGBA64 => {
                            let rgba_w: Rgba64 = rgb_w.into();
                            return Color::RGBA64(rgba_w);
                        },
                        ColorFormatName::Gray8 |
                        ColorFormatName::SRGB | ColorFormatName::SRGBf |
                        ColorFormatName::HSV | ColorFormatName::HSL |
                        ColorFormatName::CIEXYZ | ColorFormatName::CIELab =>
                        {
                            let rgbf: RgbF = rgb_w.into();
                            conversion = Color::RGBf(rgbf);
                            continue;
                        },
                        ColorFormatName::RGBA | ColorFormatName::RGBAf => {
                            let rgba_w: Rgba64 = rgb_w.into();
                            conversion = Color::RGBA64(rgba_w);
                            continue;
                        },
                    }
                },
                Color::RGBA(rgba) => {
                    match to {
                        ColorFormatName::RGBA => {
                            return conversion;
                        },
                        ColorFormatName::Gray8 => {
                            let g8: Gray8 = rgba.into();
                            return Color::Gray8(g8);
                        },
                        ColorFormatName::Gray16 => {
                            let g16: Gray16 = rgba.into();
                            return Color::Gray16(g16);
                        }
                        ColorFormatName::Grayf => {
                            let gf: GrayF = rgba.into();
                            return Color::Grayf(gf);
                        },
                        ColorFormatName::RGB => {
                            let rgb: Rgb = rgba.into();
                            return Color::RGB(rgb);
                        },
                        ColorFormatName::RGBA64 => {
                            let rgba_w: Rgba64 = rgba.into();
                            return Color::RGBA64(rgba_w);
                        },
                        ColorFormatName::RGBAf => {
                            let rgbaf: RgbaF = rgba.into();
                            return Color::RGBAf(rgbaf);
                        },
                        ColorFormatName::SRGB | ColorFormatName::RGB48 |
                        ColorFormatName::RGBf | ColorFormatName::SRGBf =>
                        {
                            let rgb: Rgb = rgba.into();
                            conversion = Color::RGB(rgb);
                            continue;
                        }
                        ColorFormatName::HSV | ColorFormatName::HSL |
                        ColorFormatName::CIEXYZ | ColorFormatName::CIELab =>
                        {
                            let rgbf: RgbF = rgba.into();
                            conversion = Color::RGBf(rgbf);
                            continue;
                        },
                    }
                },
                Color::RGBA64(rgba_w) => {
                    match to {
                        ColorFormatName::RGBA64 => {
                            return conversion;
                        },
                        ColorFormatName::Gray16 => {
                            let g16: Gray16 = rgba_w.into();
                            return Color::Gray16(g16);
                        },
                        ColorFormatName::Grayf => {
                            let gf: GrayF = rgba_w.into();
                            return Color::Grayf(gf);
                        },
                        ColorFormatName::RGB48 => {
                            let rgb_w: Rgb48 = rgba_w.into();
                            return Color::RGB48(rgb_w);
                        },
                        ColorFormatName::RGBA => {
                            let rgba: Rgba = rgba_w.into();
                            return Color::RGBA(rgba);
                        },
                        ColorFormatName::RGBf => {
                            let rgbf: RgbF = rgba_w.into();
                            return Color::RGBf(rgbf);
                        },
                        ColorFormatName::RGBAf => {
                            let rgbaf: RgbaF = rgba_w.into();
                            return Color::RGBAf(rgbaf);
                        },
                        ColorFormatName::Gray8 |
                        ColorFormatName::RGB | ColorFormatName::SRGB | ColorFormatName::SRGBf |
                        ColorFormatName::HSV | ColorFormatName::HSL |
                        ColorFormatName::CIEXYZ | ColorFormatName::CIELab => {
                            let rgbf: RgbF = rgba_w.into();
                            conversion = Color::RGBf(rgbf);
                            continue;
                        },
                    }
                },
                Color::RGBf(rgbf) => {
                    match to {
                        ColorFormatName::RGBf => {
                            return conversion;
                        },
                        ColorFormatName::Gray8 => {
                            let g8: Gray8 = rgbf.into();
                            return Color::Gray8(g8);
                        },
                        ColorFormatName::Gray16 => {
                            let g16: Gray16 = rgbf.into();
                            return Color::Gray16(g16);
                        },
                        ColorFormatName::Grayf => {
                            let gf: GrayF = rgbf.into();
                            return Color::Grayf(gf);
                        },
                        ColorFormatName::RGB => {
                            let rgb: Rgb = rgbf.into();
                            return Color::RGB(rgb);
                        }
                        ColorFormatName::SRGB => {
                            let srgb: Srgb = rgbf.into();
                            return Color::SRGB(srgb);
                        },
                        ColorFormatName::SRGBf => {
                            let srgbf: SrgbF = rgbf.into();
                            return Color::SRGBf(srgbf);
                        }
                        ColorFormatName::RGB48 => {
                            let rgb_w: Rgb48 = rgbf.into();
                            return Color::RGB48(rgb_w);
                        },
                        ColorFormatName::RGBA => {
                            let rgba: Rgba = rgbf.into();
                            return Color::RGBA(rgba);
                        },
                        ColorFormatName::RGBA64 => {
                            let rgba_w: Rgba64 = rgbf.into();
                            return Color::RGBA64(rgba_w);
                        },
                        ColorFormatName::RGBAf => {
                            let rgbaf: RgbaF = rgbf.into();
                            return Color::RGBAf(rgbaf);
                        },
                        ColorFormatName::HSV => {
                            let hsv: Hsv = rgbf.into();
                            return Color::HSV(hsv);
                        },
                        ColorFormatName::HSL => {
                            let hsl: Hsl = rgbf.into();
                            return Color::HSL(hsl);
                        },
                        ColorFormatName::CIEXYZ => {
                            let xyz: CieXyz = rgbf.into();
                            return Color::CIEXYZ(xyz);
                        },
                        ColorFormatName::CIELab => {
                            let xyz: CieXyz = rgbf.into();
                            let lab: CieLab = xyz.into();
                            return Color::CIELab(lab);
                        },
                    }
                },
                Color::SRGBf(srgbf) => {
                    match to {
                        ColorFormatName::SRGBf => {
                            return conversion;
                        },
                        ColorFormatName::Grayf => {
                            let grayf: GrayF = srgbf.into();
                            return Color::Grayf(grayf);
                        },
                        ColorFormatName::RGB => {
                            let rgb: Rgb = srgbf.into();
                            return Color::RGB(rgb);
                        },
                        ColorFormatName::SRGB => {
                            let srgb: Srgb = srgbf.into();
                            return Color::SRGB(srgb);
                        },
                        ColorFormatName::RGBf => {
                            let rgbf: RgbF = srgbf.into();
                            return Color::RGBf(rgbf);
                        },
                        ColorFormatName::HSV => {
                            let hsv: Hsv = srgbf.into();
                            return Color::HSV(hsv);
                        },
                        ColorFormatName::HSL => {
                            let hsl: Hsl = srgbf.into();
                            return Color::HSL(hsl);
                        },
                        ColorFormatName::CIEXYZ => {
                            let xyz: CieXyz = srgbf.into();
                            return Color::CIEXYZ(xyz);
                        },
                        ColorFormatName::RGBA => {
                            let rgb: Rgb = srgbf.into();
                            let rgba: Rgba = rgb.into();
                            return Color::RGBA(rgba);
                        },
                        ColorFormatName::CIELab => {
                            let xyz: CieXyz = srgbf.into();
                            let lab: CieLab = xyz.into();
                            return Color::CIELab(lab);
                        },
                        ColorFormatName::RGBAf | ColorFormatName::RGB48 | ColorFormatName::RGBA64 => {
                            let rgbf: RgbF = srgbf.into();
                            conversion = Color::RGBf(rgbf);
                            continue;
                        },
                        ColorFormatName::Gray8 | ColorFormatName::Gray16 => {
                            let grayf: GrayF = srgbf.into();
                            conversion = Color::Grayf(grayf);
                            continue;
                        },
                    }
                }
                Color::RGBAf(rgbaf) => {
                    match to {
                        ColorFormatName::RGBAf => {
                            return conversion;
                        },
                        ColorFormatName::Gray8 => {
                            let g8: Gray8 = rgbaf.into();
                            return Color::Gray8(g8);
                        }
                        ColorFormatName::Gray16 => {
                            let g16: Gray16 = rgbaf.into();
                            return Color::Gray16(g16);
                        },
                        ColorFormatName::Grayf => {
                            let gf: GrayF = rgbaf.into();
                            return Color::Grayf(gf);
                        },
                        ColorFormatName::RGB => {
                            let rgb: Rgb = rgbaf.into();
                            return Color::RGB(rgb);
                        }
                        ColorFormatName::SRGB => {
                            let srgb: Srgb = rgbaf.into();
                            return Color::SRGB(srgb);
                        },
                        ColorFormatName::RGB48 => {
                            let rgb_w: Rgb48 = rgbaf.into();
                            return Color::RGB48(rgb_w);
                        },
                        ColorFormatName::RGBA => {
                            let rgba: Rgba = rgbaf.into();
                            return Color::RGBA(rgba);
                        },
                        ColorFormatName::RGBA64 => {
                            let rgba_w: Rgba64 = rgbaf.into();
                            return Color::RGBA64(rgba_w);
                        },
                        ColorFormatName::RGBf => {
                            let rgbf: RgbF = rgbaf.into();
                            return Color::RGBf(rgbf);
                        },
                        ColorFormatName::SRGBf => {
                            let srgbf: SrgbF = rgbaf.into();
                            return Color::SRGBf(srgbf);
                        }
                        ColorFormatName::HSV => {
                            let hsv: Hsv = rgbaf.into();
                            return Color::HSV(hsv);
                        },
                        ColorFormatName::HSL => {
                            let hsl: Hsl = rgbaf.into();
                            return Color::HSL(hsl);
                        },
                        ColorFormatName::CIEXYZ => {
                            let xyz: CieXyz = rgbaf.into();
                            return Color::CIEXYZ(xyz);
                        },
                        ColorFormatName::CIELab => {
                            let xyz: CieXyz = rgbaf.into();
                            let lab: CieLab = xyz.into();
                            return Color::CIELab(lab);
                        },
                    }
                },
                Color::HSV(hsv) => {
                    match to {
                        ColorFormatName::HSV => {
                            return conversion;
                        },
                        ColorFormatName::RGB => {
                            let rgb: Rgb = hsv.into();
                            return Color::RGB(rgb);
                        },
                        ColorFormatName::SRGB => {
                            let srgb: Srgb = hsv.into();
                            return Color::SRGB(srgb);
                        },
                        ColorFormatName::RGBf => {
                            let rgbf: RgbF = hsv.into();
                            return Color::RGBf(rgbf);
                        },
                        ColorFormatName::SRGBf => {
                            let srgbf: SrgbF = hsv.into();
                            return Color::SRGBf(srgbf);
                        }
                        ColorFormatName::HSL => {
                            let hsl: Hsl = hsv.into();
                            return Color::HSL(hsl);
                        },
                        ColorFormatName::Gray8 | ColorFormatName::Gray16 | ColorFormatName::Grayf |
                        ColorFormatName::RGB48 | ColorFormatName::RGBA64 |
                        ColorFormatName::RGBA | ColorFormatName::RGBAf |
                        ColorFormatName::CIEXYZ | ColorFormatName::CIELab => {
                            let rgbf: RgbF = hsv.into();
                            conversion = Color::RGBf(rgbf);
                            continue;
                        },
                    }
                },
                Color::HSL(hsl) => {
                    match to {
                        ColorFormatName::HSL => {
                            return conversion;
                        },
                        ColorFormatName::Gray8 | ColorFormatName::Gray16 | ColorFormatName::Grayf =>
                        {

                        },
                        ColorFormatName::RGB => {
                            let rgb: Rgb = hsl.into();
                            return Color::RGB(rgb);
                        },
                        ColorFormatName::SRGB => {
                            let srgb: Srgb = hsl.into();
                            return Color::SRGB(srgb);
                        },
                        ColorFormatName::RGBf => {
                            let rgbf: RgbF = hsl.into();
                            return Color::RGBf(rgbf);
                        },
                        ColorFormatName::SRGBf => {
                            let srgbf: SrgbF = hsl.into();
                            return Color::SRGBf(srgbf);
                        }
                        ColorFormatName::HSV => {
                            let hsv: Hsv = hsl.into();
                            return Color::HSV(hsv);
                        },
                        ColorFormatName::RGB48 | ColorFormatName::RGBA64 |
                        ColorFormatName::RGBA | ColorFormatName::RGBAf |
                        ColorFormatName::CIEXYZ | ColorFormatName::CIELab => {
                            let rgbf: RgbF = hsl.into();
                            conversion = Color::RGBf(rgbf);
                            continue;
                        },
                    }
                },
                Color::CIEXYZ(xyz) => {
                    match to {
                        ColorFormatName::CIEXYZ => {
                            return conversion;
                        },
                        ColorFormatName::Grayf => {
                            let gf: GrayF = xyz.into();
                            return Color::Grayf(gf);
                        },
                        ColorFormatName::RGB => {
                            let rgb: Rgb = xyz.into();
                            return Color::RGB(rgb);
                        },
                        ColorFormatName::SRGB => {
                            let srgb: Srgb = xyz.into();
                            return Color::SRGB(srgb);
                        },
                        ColorFormatName::RGBf => {
                            let rgbf: RgbF = xyz.into();
                            return Color::RGBf(rgbf);
                        },
                        ColorFormatName::SRGBf => {
                            let srgbf: SrgbF = xyz.into();
                            return Color::SRGBf(srgbf);
                        }
                        ColorFormatName::CIELab => {
                            let lab: CieLab = xyz.into();
                            return Color::CIELab(lab);
                        },
                        ColorFormatName::RGBA => {
                            let rgb: Rgb = xyz.into();
                            let rgba: Rgba = rgb.into();
                            return Color::RGBA(rgba);
                        },
                        ColorFormatName::Gray8 | ColorFormatName::Gray16 => {
                            let gf: GrayF = xyz.into();
                            conversion = Color::Grayf(gf);
                            continue;
                        },
                        ColorFormatName::RGBAf | ColorFormatName::RGB48 |ColorFormatName::RGBA64 |
                        ColorFormatName::HSV | ColorFormatName::HSL => {
                            let rgbf: RgbF = xyz.into();
                            conversion = Color::RGBf(rgbf);
                            continue;
                        },
                    }
                },
                Color::CIELab(lab) => {
                    match to {
                        ColorFormatName::CIELab => {
                            return conversion;
                        },
                        ColorFormatName::Grayf => {
                            let gf: GrayF = lab.into();
                            return Color::Grayf(gf);
                        },
                        ColorFormatName::CIEXYZ => {
                            let xyz: CieXyz = lab.into();
                            return Color::CIEXYZ(xyz);
                        },
                        ColorFormatName::Gray8 | ColorFormatName::Gray16 => {
                            let gf: GrayF = lab.into();
                            conversion = Color::Grayf(gf);
                            continue;
                        },
                        ColorFormatName::RGB | ColorFormatName::SRGB | ColorFormatName::RGBA |
                        ColorFormatName::RGB48 | ColorFormatName::RGBA64 |
                        ColorFormatName::RGBf | ColorFormatName::SRGBf | ColorFormatName::RGBAf |
                        ColorFormatName::HSV | ColorFormatName::HSL=>
                        {
                            let xyz: CieXyz = lab.into();
                            conversion = Color::CIEXYZ(xyz);
                            continue;
                        },
                    }
                },
            }
        }
    }

}
