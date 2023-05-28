use super::*;

//Note: Grayscale formats are *linear* luminance (Y) rather than gamma-compressed luma (Y')
//Equivalent to CIEXYZ's Y component
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Gray8 {
    pub luminance: u8
}

impl Gray8 {
    pub fn new(intensity: u8) -> Gray8 {
        Gray8 {
            luminance: intensity
        }
    }
}

impl ColorFormat for Gray8 {

    fn channel_count() -> u8 {
        1u8
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::Gray8
    }

    fn to_bytes(self) -> Vec<u8> {
        vec![self.luminance]
    }

    fn as_bytes(&self) -> Vec<u8> {
        vec![self.luminance]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 1 {
            let g8 = Gray8 { luminance: bytes[0] };
            Ok(g8)
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::Gray8, vec![self.luminance])
    }
}

impl From<Gray16> for Gray8 {
    fn from(g16: Gray16) -> Gray8 {
        let g8 = ((g16.luminance as f32 / (u16::MAX as f32)) * (u8::MAX as f32)) as u8;
        Gray8::new(g8)
    }
}

impl From<GrayF> for Gray8 {
    fn from(gf: GrayF) -> Gray8 {
        let g8 = (gf.luminance * (u8::MAX as f32)) as u8;
        Gray8::new(g8)
    }
}

impl From<Rgb> for Gray8 {
    fn from(rgb: Rgb) -> Gray8 {
        let r_f = rgb.r as f32;
        let g_f = rgb.g as f32;
        let b_f = rgb.b as f32;

        let lum = gray_from_rgb(r_f, g_f, b_f);
        let lum = lum as u8;

        Gray8::new(lum)
    }
}

impl From<Rgba> for Gray8 {
    fn from(rgba: Rgba) -> Gray8 {
        let r_f = rgba.r as f32;
        let g_f = rgba.g as f32;
        let b_f = rgba.b as f32;

        let lum = gray_from_rgb(r_f, g_f, b_f);
        let lum = lum as u8;

        Gray8::new(lum)
    }
}

impl From<RgbF> for Gray8 {
    fn from(rgbf: RgbF) -> Gray8 {
        let lum = gray_from_rgb(rgbf.r, rgbf.g, rgbf.b);
        let lum = (lum.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;

        Gray8::new(lum)
    }
}

impl From<RgbaF> for Gray8 {
    fn from(rgbaf: RgbaF) -> Gray8 {
        let lum = gray_from_rgb(rgbaf.r, rgbaf.g, rgbaf.b);
        let lum = (lum * (u8::MAX as f32)) as u8;

        Gray8::new(lum)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Gray16 {
    pub luminance: u16
}

impl Gray16 {
    pub fn new(intensity: u16) -> Gray16 {
        Gray16 {
            luminance: intensity
        }
    }
}

impl ColorFormat for Gray16 {

    fn channel_count() -> u8 {
        1u8
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::Gray16
    }

    fn to_bytes(self) -> Vec<u8> {
        let bytes = u16::to_ne_bytes(self.luminance);
        vec![bytes[0], bytes[1]]
    }

    fn as_bytes(&self) -> Vec<u8> {
        let bytes = u16::to_ne_bytes(self.luminance);
        vec![bytes[0], bytes[1]]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 2 {
            let g16 = Gray16 {
                luminance: u16::from_ne_bytes([bytes[0], bytes[1]])
            };
            Ok(g16)
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::Gray16, self.to_bytes())
    }
}

impl From<Gray8> for Gray16 {
    fn from(g8: Gray8) -> Gray16 {
        let g16 = ((g8.luminance as f32 / (u8::MAX as f32)) * (u16::MAX as f32)) as u16;
        Gray16::new(g16)
    }
}

impl From<GrayF> for Gray16 {
    fn from(gf: GrayF) -> Gray16 {
        let g16 = (gf.luminance * (u16::MAX as f32)) as u16;
        Gray16::new(g16)
    }
}

impl From<Rgb> for Gray16 {
    fn from(rgb: Rgb) -> Gray16 {
        let r_f = rgb.r as f32;
        let g_f = rgb.g as f32;
        let b_f = rgb.b as f32;

        let lum = gray_from_rgb(r_f, g_f, b_f);
        let lum = (lum * ((u16::MAX as f32) / (u8::MAX as f32))) as u16;

        Gray16::new(lum)
    }
}

impl From<Rgba> for Gray16 {
    fn from(rgba: Rgba) -> Gray16 {
        let r_f = rgba.r as f32;
        let g_f = rgba.g as f32;
        let b_f = rgba.b as f32;

        let lum = gray_from_rgb(r_f, g_f, b_f);
        let lum = (lum * ((u16::MAX as f32) / (u8::MAX as f32))) as u16;

        Gray16::new(lum)
    }
}

impl From<Rgb48> for Gray16 {
    fn from(rgb_w: Rgb48) -> Gray16 {
        let r_f = rgb_w.r as f32;
        let g_f = rgb_w.g as f32;
        let b_f = rgb_w.b as f32;

        let lum = gray_from_rgb(r_f, g_f, b_f);
        let lum = lum as u16;

        Gray16::new(lum)
    }
}

impl From<Rgba64> for Gray16 {
    fn from(rgba_w: Rgba64) -> Gray16 {
        let r_f = rgba_w.r as f32;
        let g_f = rgba_w.g as f32;
        let b_f = rgba_w.b as f32;

        let lum = gray_from_rgb(r_f, g_f, b_f);
        let lum = lum as u16;

        Gray16::new(lum)
    }
}

impl From<RgbF> for Gray16 {
    fn from(rgbf: RgbF) -> Gray16 {
        let lum = gray_from_rgb(rgbf.r, rgbf.g, rgbf.b);
        let lum = (lum.clamp(0.0f32, 1.0f32) * (u16::MAX as f32)) as u16;

        Gray16::new(lum)
    }
}

impl From<RgbaF> for Gray16 {
    fn from(rgbaf: RgbaF) -> Gray16 {
        let lum = gray_from_rgb(rgbaf.r, rgbaf.g, rgbaf.b);
        let lum = (lum * (u16::MAX as f32)) as u16;

        Gray16::new(lum)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct GrayF {
    pub luminance: f32
}

impl GrayF {
    pub fn new(intensity: f32) -> GrayF {
        GrayF {
            luminance: intensity
        }
    }
}

impl ColorFormat for GrayF {

    fn channel_count() -> u8 {
        1u8
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::Grayf
    }

    fn to_bytes(self) -> Vec<u8> {
        let bytes = f32::to_ne_bytes(self.luminance);
        vec![bytes[0], bytes[1], bytes[2], bytes[3]]
    }

    fn as_bytes(&self) -> Vec<u8> {
        let bytes = f32::to_ne_bytes(self.luminance);
        vec![bytes[0], bytes[1], bytes[2], bytes[3]]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 4 {
            let gf = GrayF {
                luminance: f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
            };
            Ok(gf)
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::Grayf, self.to_bytes())
    }
}

impl From<Gray8> for GrayF {
    fn from(g8: Gray8) -> GrayF {
        let g_f = g8.luminance as f32 / (u8::MAX as f32);
        GrayF::new(g_f)
    }
}

impl From<Gray16> for GrayF {
    fn from(g16: Gray16) -> GrayF {
        let g_f = g16.luminance as f32 / (u16::MAX as f32);

        GrayF::new(g_f)
    }
}

impl From<Rgb> for GrayF {
    fn from(rgb: Rgb) -> GrayF {
        let (r_f, g_f, b_f) = (rgb.r as f32, rgb.g as f32, rgb.b as f32);
        let lum = gray_from_rgb(r_f, g_f, b_f);
        let lum = lum / (u8::MAX as f32);

        GrayF::new(lum)
    }
}

impl From<Srgb> for GrayF {
    fn from(srgb: Srgb) -> GrayF {
        let (r_f, g_f, b_f) = (srgb.r as f32, srgb.g as f32, srgb.b as f32);
        let (r_f, g_f, b_f) = (r_f / (u8::MAX as f32), g_f / (u8::MAX as f32), b_f / (u8::MAX as f32));

        let rl_f = srgb_gamma_expand(r_f);
        let gl_f = srgb_gamma_expand(g_f);
        let bl_f = srgb_gamma_expand(b_f);

        let lum = gray_from_rgb(rl_f, gl_f, bl_f);

        GrayF::new(lum)
    }
}

impl From<SrgbF> for GrayF {
    fn from(srgb_f: SrgbF) -> GrayF {
        let rl_f = srgb_gamma_expand(srgb_f.r);
        let gl_f = srgb_gamma_expand(srgb_f.g);
        let bl_f = srgb_gamma_expand(srgb_f.b);

        let lum = gray_from_rgb(rl_f, gl_f, bl_f);

        GrayF::new(lum)
    }
}

impl From<Rgb48> for GrayF {
    fn from(rgb_w: Rgb48) -> GrayF {
        let (r_f, g_f, b_f) = (rgb_w.r as f32, rgb_w.g as f32, rgb_w.b as f32);
        let lum = gray_from_rgb(r_f, g_f, b_f);
        let lum = lum / (u16::MAX as f32);

        GrayF::new(lum)
    }
}

impl From<Rgba> for GrayF {
    fn from(rgba: Rgba) -> GrayF {
        let (r_f, g_f, b_f) = (rgba.r as f32, rgba.g as f32, rgba.b as f32);
        let lum = gray_from_rgb(r_f, g_f, b_f);
        let lum = lum / (u8::MAX as f32);

        GrayF::new(lum)
    }
}

impl From<Rgba64> for GrayF {
    fn from(rgba_w: Rgba64) -> GrayF {
        let (r_f, g_f, b_f) = (rgba_w.r as f32, rgba_w.g as f32, rgba_w.b as f32);
        let lum = gray_from_rgb(r_f, g_f, b_f);
        let lum = lum / (u16::MAX as f32);

        GrayF::new(lum)
    }
}

impl From<RgbF> for GrayF {
    fn from(rgbf: RgbF) -> GrayF {
        let lum = gray_from_rgb(rgbf.r, rgbf.g, rgbf.b);

        GrayF::new(lum)
    }
}

impl From<RgbaF> for GrayF {
    fn from(rgbaf: RgbaF) -> GrayF {
        let lum = gray_from_rgb(rgbaf.r, rgbaf.g, rgbaf.b);

        GrayF::new(lum)
    }
}

impl From<CieXyz> for GrayF {
    fn from(xyz: CieXyz) -> GrayF {
        let xyz = if let Some(ref_white) = xyz.ref_xyz {
            let def_white = CieXyz::default();
            let ref_white = CieXyz::new(ref_white.0, ref_white.1, ref_white.2);

            if ref_white != def_white {
                CieXyz::chromatic_adaptation(xyz, ref_white)
            } else {
                xyz
            }
        } else {
            xyz
        };

        GrayF::new(xyz.y)
    }
}

impl From<CieLab> for GrayF {
    fn from(lab: CieLab) -> GrayF {
        let epsilon = 0.008856f32;
        let kappa = 903.3f32;

        let f_y = (lab.l + 16.0f32) / 116.0f32;

        let y_r = if lab.l > kappa * epsilon {
            f_y.powi(3)
        } else {
            lab.l / kappa
        };

        let ref_y = if let Some(triplet) = lab.ref_xyz {
            triplet.1
        } else {
            1.0f32
        };

        GrayF::new((y_r * ref_y).clamp(0.0f32, 1.0f32))
    }
}
