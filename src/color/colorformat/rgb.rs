use super::*;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb {
            r: r,
            g: g,
            b: b,
        }
    }
}

impl ColorFormat for Rgb {

    fn channel_count() -> u8 {
        3
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::RGB
    }

    fn to_bytes(self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }

    fn as_bytes(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::RGB, self.to_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 3 {
            let rgb = Rgb::new(bytes[0], bytes[1], bytes[2]);
            Ok(rgb)
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                            .to_owned())
        }
    }
}

impl From<Gray8> for Rgb {
    fn from(g8: Gray8) -> Rgb {
        Rgb::new(g8.luminance, g8.luminance, g8.luminance)
    }
}

impl From<Gray16> for Rgb {
    fn from(g16: Gray16) -> Rgb {
        let g_f = (g16.luminance as f32) / ((u16::MAX as f32) / (u8::MAX as f32));
        let g8 = g_f as u8;
        Rgb::new(g8, g8, g8)
    }
}

impl From<GrayF> for Rgb {
    fn from(gf: GrayF) -> Rgb {
        let g8 = (gf.luminance * (u8::MAX as f32)) as u8;
        Rgb::new(g8, g8, g8)
    }
}

impl From<Srgb> for Rgb {
    fn from(srgb: Srgb) -> Rgb {
        let (r_f, g_f, b_f) = (srgb.r as f32, srgb.g as f32, srgb.b as f32);
        let (r_f, g_f, b_f) = (r_f / (u8::MAX as f32), g_f / (u8::MAX as f32), b_f / (u8::MAX as f32));

        let rl_f = srgb_gamma_expand(r_f);
        let gl_f = srgb_gamma_expand(g_f);
        let bl_f = srgb_gamma_expand(b_f);

        let (r8, g8, b8) = ((rl_f * (u8::MAX as f32)) as u8, (gl_f * (u8::MAX as f32)) as u8, (bl_f * (u8::MAX as f32)) as u8);

        Rgb::new(r8, g8, b8)
    }
}

impl From<SrgbF> for Rgb {
    fn from(srgb_f: SrgbF) -> Rgb {
        let (r_f, g_f, b_f) = (srgb_f.r, srgb_f.g, srgb_f.b);
        let (r_f, g_f, b_f) = (r_f.clamp(0.0f32, 1.0f32), g_f.clamp(0.0f32, 1.0f32), b_f.clamp(0.0f32, 1.0f32));
        let (r_f, g_f, b_f) = (r_f / (u8::MAX as f32), g_f / (u8::MAX as f32), b_f / (u8::MAX as f32));

        let rl_f = srgb_gamma_expand(r_f);
        let gl_f = srgb_gamma_expand(g_f);
        let bl_f = srgb_gamma_expand(b_f);

        let (r8, g8, b8) = ((rl_f * (u8::MAX as f32)) as u8, (gl_f * (u8::MAX as f32)) as u8, (bl_f * (u8::MAX as f32)) as u8);

        Rgb::new(r8, g8, b8)
    }
}

impl From<Rgb48> for Rgb {
    fn from(rgb_w: Rgb48) -> Rgb {
        let r_f = rgb_w.r as f32 / (u16::MAX as f32);
        let g_f = rgb_w.g as f32 / (u16::MAX as f32);
        let b_f = rgb_w.b as f32 / (u16::MAX as f32);

        let r8 = (r_f * (u8::MAX as f32)) as u8;
        let g8 = (g_f * (u8::MAX as f32)) as u8;
        let b8 = (b_f * (u8::MAX as f32)) as u8;

        Rgb::new(r8, g8, b8)
    }
}

impl From<Rgba> for Rgb {
    fn from(rgba: Rgba) -> Rgb {
        Rgb::new(rgba.r, rgba.g, rgba.b)
    }
}

impl From<RgbF> for Rgb {
    fn from(rgbf: RgbF) -> Rgb {
        let r8 = (rgbf.r.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;
        let g8 = (rgbf.g.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;
        let b8 = (rgbf.b.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;

        Rgb::new(r8, g8, b8)
    }
}

impl From<RgbaF> for Rgb {
    fn from(rgbaf: RgbaF) -> Rgb {
        let r8 = (rgbaf.r.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;
        let g8 = (rgbaf.g.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;
        let b8 = (rgbaf.b.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;

        Rgb::new(r8, g8, b8)
    }
}

impl From<Hsv> for Rgb {
    fn from(hsv: Hsv) -> Rgb {
        let c = hsv.s * hsv.v;
        let h = hsv.h / 60.0f32;
        let x = c * (1.0f32 - ((h % 2.0f32) - 1.0f32).abs());

        let (r, g, b) =
            if 0.0f32 <= h && h <= 1.0f32 {
                (c, x, 0.0f32)
            } else if 1.0f32 < h && h <= 2.0f32 {
                (x, c, 0.0f32)
            } else if 2.0f32 < h && h <= 3.0f32 {
                (0.0f32, c, x)
            } else if 3.0f32 < h && h <= 4.0f32 {
                (0.0f32, x, c)
            } else if 4.0f32 < h && h <= 5.0f32 {
                (x, 0.0f32, c)
            } else if 5.0f32 < h && h <= 6.0f32 {
                (c, 0.0f32, x)
            } else {
                (0.0f32, 0.0f32, 0.0f32)
            };

        let m = hsv.v - c;
        let (r_f, g_f, b_f) = (r + m, g + m, b + m);

        let r8 = (r_f * (u8::MAX as f32)) as u8;
        let g8 = (g_f * (u8::MAX as f32)) as u8;
        let b8 = (b_f * (u8::MAX as f32)) as u8;

        Rgb::new(r8, g8, b8)
    }
}

impl From<Hsl> for Rgb {
    fn from(hsl: Hsl) -> Rgb {
        let c = (1.0f32 - ((2.0f32 * hsl.l) - 1.0f32).abs()) * hsl.s;
        let h = hsl.h / 60.0f32;
        let x = c * (1.0f32 - ((h % 2.0f32) - 1.0f32).abs());

        let (r, g, b) =
            if 0.0f32 <= h && h < 1.0f32 {
                (c, x, 0.0f32)
            } else if 1.0f32 <= h && h < 2.0f32 {
                (x, c, 0.0f32)
            } else if 2.0f32 <= h && h < 3.0f32 {
                (0.0f32, c, x)
            } else if 3.0f32 <= h && h < 4.0f32 {
                (0.0f32, x, c)
            } else if 4.0f32 <= h && h < 5.0f32 {
                (x, 0.0f32, c)
            } else if 5.0f32 <= h && h < 6.0f32 {
                (c, 0.0f32, x)
            } else {
                (0.0f32, 0.0f32, 0.0f32)
            };

        let m = hsl.l - (c / 2.0f32);
        let (r_f, g_f, b_f) = (r + m, g + m, b + m);

        let r8 = (r_f * (u8::MAX as f32)) as u8;
        let g8 = (g_f * (u8::MAX as f32)) as u8;
        let b8 = (b_f * (u8::MAX as f32)) as u8;

        Rgb::new(r8, g8, b8)
    }
}

impl From<CieXyz> for Rgb {
    fn from(xyz: CieXyz) -> Rgb {
        let dest_ref_x = 0.95047f32;
        let dest_ref_y = 1.0f32;
        let dest_ref_z = 1.08883f32;

        let dest_ref_xyz = CieXyz::new(dest_ref_x, dest_ref_y, dest_ref_z);

        let src_ref_xyz =  if let Some(triplet) = xyz.ref_xyz {
            CieXyz::new(triplet.0, triplet.1, triplet.2)
        } else {
            dest_ref_xyz
        };

        //Don't need to do chromatic adaptation unless the reference white is different
        let xyz = if src_ref_xyz.x != dest_ref_xyz.x ||
                     src_ref_xyz.y != dest_ref_xyz.y ||
                     src_ref_xyz.z != dest_ref_xyz.z
        {
            CieXyz::chromatic_adaptation(xyz.clone(), dest_ref_xyz)
        } else {
            xyz
        };

        let xyz = [xyz.x, xyz.y, xyz.z];

        let m_inverse = [[3.2404542f32, -1.5371385f32, -0.4985314f32],
                         [-0.9692660f32, 1.8760108f32, 0.0415560f32],
                         [0.0556434f32, -0.2040259f32, 1.0572252f32]];

        let mut rgb = [0.0f32, 0.0f32, 0.0f32];

        for i in 0..rgb.len() {
            let mut row_sum = 0.0f32;
            for j in 0..xyz.len() {
                row_sum += m_inverse[i][j] * xyz[j];
            }
            rgb[i] += row_sum;
        }

        let r = (rgb[0].clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;
        let g = (rgb[1].clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;
        let b = (rgb[2].clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;

        Rgb::new(r, g, b)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Srgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Srgb {
    pub fn new(sr: u8, sg: u8, sb: u8) -> Srgb {
        Srgb {
            r: sr,
            g: sg,
            b: sb,
        }
    }
}

impl ColorFormat for Srgb {

    fn channel_count() -> u8 {
        3
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::SRGB
    }

    fn to_bytes(self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }

    fn as_bytes(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::SRGB, self.to_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 3 {
            let srgb = Srgb::new(bytes[0], bytes[1], bytes[2]);
            Ok(srgb)
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                                Hint: Should be exactly {} byte(s). {} bytes were provided.",
                                type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                                .to_owned())
        }
    }
}

impl From<GrayF> for Srgb {
    fn from(gf: GrayF) -> Srgb {
        let g_prime = srgb_gamma_compress(gf.luminance);
        let g = (g_prime * (u8::MAX as f32)) as u8;

        Srgb::new(g, g, g)
    }
}

impl From<Rgb> for Srgb {
    fn from(rgb: Rgb) -> Srgb {
        let (r_f, g_f, b_f) = (rgb.r as f32, rgb.g as f32, rgb.b as f32);
        let (r_f, g_f, b_f) = ((r_f / (u8::MAX as f32)), (g_f / (u8::MAX as f32)), (b_f / (u8::MAX as f32)));

        let rl_f = srgb_gamma_compress(r_f);
        let gl_f = srgb_gamma_compress(g_f);
        let bl_f = srgb_gamma_compress(b_f);

        let (sr8, sg8, sb8) = ((rl_f * (u8::MAX as f32)) as u8, (gl_f * (u8::MAX as f32)) as u8, (bl_f * (u8::MAX as f32)) as u8);

        Srgb::new(sr8, sg8, sb8)
    }
}

impl From<RgbF> for Srgb {
    fn from(rgbf: RgbF) -> Srgb {
        let rl_f = srgb_gamma_compress(rgbf.r.clamp(0.0f32, 1.0f32)) * (u8::MAX as f32);
        let gl_f = srgb_gamma_compress(rgbf.g.clamp(0.0f32, 1.0f32)) * (u8::MAX as f32);
        let bl_f = srgb_gamma_compress(rgbf.b.clamp(0.0f32, 1.0f32)) * (u8::MAX as f32);

        let (sr8, sg8, sb8) = (rl_f as u8, gl_f as u8, bl_f as u8);

        Srgb::new(sr8, sg8, sb8)
    }
}

impl From<SrgbF> for Srgb {
    fn from(srgbf: SrgbF) -> Srgb {
        let sr8 = (srgbf.r.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;
        let sg8 = (srgbf.g.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;
        let sb8 = (srgbf.b.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;

        Srgb::new(sr8, sg8, sb8)
    }
}

impl From<RgbaF> for Srgb {
    fn from(rgbaf: RgbaF) -> Srgb {
        let rl_f = srgb_gamma_compress(rgbaf.r) * (u8::MAX as f32);
        let gl_f = srgb_gamma_compress(rgbaf.g) * (u8::MAX as f32);
        let bl_f = srgb_gamma_compress(rgbaf.b) * (u8::MAX as f32);

        let (sr8, sg8, sb8) = (rl_f as u8, gl_f as u8, bl_f as u8);

        Srgb::new(sr8, sg8, sb8)
    }
}

impl From<Hsv> for Srgb {
    fn from(hsv: Hsv) -> Srgb {
        let c = hsv.s * hsv.v;
        let h = hsv.h / 60.0f32;
        let x = c * (1.0f32 - ((h % 2.0f32) - 1.0f32).abs());

        let (r, g, b) =
            if 0.0f32 <= h && h <= 1.0f32 {
                (c, x, 0.0f32)
            } else if 1.0f32 < h && h <= 2.0f32 {
                (x, c, 0.0f32)
            } else if 2.0f32 < h && h <= 3.0f32 {
                (0.0f32, c, x)
            } else if 3.0f32 < h && h <= 4.0f32 {
                (0.0f32, x, c)
            } else if 4.0f32 < h && h <= 5.0f32 {
                (x, 0.0f32, c)
            } else if 5.0f32 < h && h <= 6.0f32 {
                (c, 0.0f32, x)
            } else {
                (0.0f32, 0.0f32, 0.0f32)
            };

        let m = hsv.v - c;
        let (r_l, g_l, b_l) = (r + m, g + m, b + m);

        let sr_f = srgb_gamma_compress(r_l.clamp(0.0f32, 1.0f32));
        let sg_f = srgb_gamma_compress(g_l.clamp(0.0f32, 1.0f32));
        let sb_f = srgb_gamma_compress(b_l.clamp(0.0f32, 1.0f32));

        let sr = (sr_f * (u8::MAX as f32)) as u8;
        let sg = (sg_f * (u8::MAX as f32)) as u8;
        let sb = (sb_f * (u8::MAX as f32)) as u8;

        Srgb::new(sr, sg, sb)
    }
}

impl From<Hsl> for Srgb {
    fn from(hsl: Hsl) -> Srgb {
        let c = (1.0f32 - ((2.0f32 * hsl.l) - 1.0f32).abs()) * hsl.s;
        let h = hsl.h / 60.0f32;
        let x = c * (1.0f32 - ((h % 2.0f32) - 1.0f32).abs());

        let (r, g, b) =
            if 0.0f32 <= h && h < 1.0f32 {
                (c, x, 0.0f32)
            } else if 1.0f32 <= h && h < 2.0f32 {
                (x, c, 0.0f32)
            } else if 2.0f32 <= h && h < 3.0f32 {
                (0.0f32, c, x)
            } else if 3.0f32 <= h && h < 4.0f32 {
                (0.0f32, x, c)
            } else if 4.0f32 <= h && h < 5.0f32 {
                (x, 0.0f32, c)
            } else if 5.0f32 <= h && h < 6.0f32 {
                (c, 0.0f32, x)
            } else {
                (0.0f32, 0.0f32, 0.0f32)
            };

        let m = hsl.l - (c / 2.0f32);
        let (r_l, g_l, b_l) = (r + m, g + m, b + m);

        let sr_f = srgb_gamma_compress(r_l.clamp(0.0f32, 1.0f32));
        let sg_f = srgb_gamma_compress(g_l.clamp(0.0f32, 1.0f32));
        let sb_f = srgb_gamma_compress(b_l.clamp(0.0f32, 1.0f32));

        let sr = (sr_f * (u8::MAX as f32)) as u8;
        let sg = (sg_f * (u8::MAX as f32)) as u8;
        let sb = (sb_f * (u8::MAX as f32)) as u8;

        Srgb::new(sr, sg, sb)
    }
}

impl From<CieXyz> for Srgb {
    fn from(xyz: CieXyz) -> Srgb {

        //sRGB reference white (D65)
        let dest_ref_x = 0.95047f32;
        let dest_ref_y = 1.0f32;
        let dest_ref_z = 1.08883f32;

        let dest_ref_xyz = CieXyz::new(dest_ref_x, dest_ref_y, dest_ref_z);

        let src_ref_xyz =  if let Some(triplet) = xyz.ref_xyz {
            CieXyz::new(triplet.0, triplet.1, triplet.2)
        } else {
            dest_ref_xyz
        };

        //Don't need to do chromatic adaptation unless the reference white is different
        let xyz = if src_ref_xyz.x != dest_ref_xyz.x ||
                     src_ref_xyz.y != dest_ref_xyz.y ||
                     src_ref_xyz.z != dest_ref_xyz.z
        {
            CieXyz::chromatic_adaptation(xyz.clone(), dest_ref_xyz)
        } else {
            xyz
        };

        let xyz = [xyz.x, xyz.y, xyz.z];

        let m_inverse = [[3.2404542f32, -1.5371385f32, -0.4985314f32],
                         [-0.9692660f32, 1.8760108f32, 0.0415560f32],
                         [0.0556434f32, -0.2040259f32, 1.0572252f32]];

        let mut rgb = [0.0f32, 0.0f32, 0.0f32];

        for i in 0..rgb.len() {
            let mut row_sum = 0.0f32;
            for j in 0..xyz.len() {
                row_sum += m_inverse[i][j] * xyz[j];
            }
            rgb[i] += row_sum;
        }

        for i in 0..rgb.len() {
            rgb[i] = srgb_gamma_compress(rgb[i]);
        }

        let sr = (rgb[0].clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;
        let sg = (rgb[1].clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;
        let sb = (rgb[2].clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;

        Srgb::new(sr, sg, sb)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Rgb48 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

impl Rgb48 {
    pub fn new(r: u16, g: u16, b: u16) -> Rgb48 {
        Rgb48 {
            r: r,
            g: g,
            b: b,
        }
    }
}

impl ColorFormat for Rgb48 {

    fn channel_count() -> u8 {
        3
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::RGB48
    }

    fn to_bytes(self) -> Vec<u8> {
        let r16 = u16::to_ne_bytes(self.r);
        let g16 = u16::to_ne_bytes(self.g);
        let b16 = u16::to_ne_bytes(self.b);

        vec![r16[0], r16[1],
             g16[0], g16[1],
             b16[0], b16[1]]
    }

    fn as_bytes(&self) -> Vec<u8> {
        let r16 = u16::to_ne_bytes(self.r);
        let g16 = u16::to_ne_bytes(self.g);
        let b16 = u16::to_ne_bytes(self.b);

        vec![r16[0], r16[1],
             g16[0], g16[1],
             b16[0], b16[1]]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 6 {
            let r16 = u16::from_ne_bytes([bytes[0], bytes[1]]);
            let g16 = u16::from_ne_bytes([bytes[2], bytes[3]]);
            let b16 = u16::from_ne_bytes([bytes[4], bytes[5]]);
            Ok(Rgb48::new(r16, g16, b16))
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::RGB48, self.to_bytes())
    }
}

impl From<Gray16> for Rgb48 {
    fn from(g16: Gray16) -> Rgb48 {
        Rgb48::new(g16.luminance, g16.luminance, g16.luminance)
    }
}

impl From<GrayF> for Rgb48 {
    fn from(gf: GrayF) -> Rgb48 {
        let g16 = (gf.luminance * 65536.0f32) as u16;
        Rgb48::new(g16, g16, g16)
    }
}

impl From<Rgb> for Rgb48 {
    fn from(rgb: Rgb) -> Rgb48 {
        let r_f = rgb.r as f32 * ((u16::MAX as f32) / (u8::MAX as f32));
        let g_f = rgb.g as f32 * ((u16::MAX as f32) / (u8::MAX as f32));
        let b_f = rgb.b as f32 * ((u16::MAX as f32) / (u8::MAX as f32));

        let r16 = r_f as u16;
        let g16 = g_f as u16;
        let b16 = b_f as u16;

        Rgb48::new(r16, g16, b16)
    }
}

impl From<Rgba64> for Rgb48 {
    fn from(rgba_w: Rgba64) -> Rgb48 {
        Rgb48::new(rgba_w.r, rgba_w.g, rgba_w.b)
    }
}

impl From<RgbF> for Rgb48 {
    fn from(rgbf: RgbF) -> Rgb48 {
        let r16 = (rgbf.r.clamp(0.0f32, 1.0f32) * (u16::MAX as f32)) as u16;
        let g16 = (rgbf.g.clamp(0.0f32, 1.0f32) * (u16::MAX as f32)) as u16;
        let b16 = (rgbf.b.clamp(0.0f32, 1.0f32) * (u16::MAX as f32)) as u16;

        Rgb48::new(r16, g16, b16)
    }
}

impl From<RgbaF> for Rgb48 {
    fn from(rgbaf: RgbaF) -> Rgb48 {
        let r16 = (rgbaf.r * (u16::MAX as f32)) as u16;
        let g16 = (rgbaf.g * (u16::MAX as f32)) as u16;
        let b16 = (rgbaf.b * (u16::MAX as f32)) as u16;

        Rgb48::new(r16, g16, b16)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct RgbF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RgbF {
    pub fn new(r: f32, g: f32, b: f32) -> RgbF {
        let r = r;
        let g = g;
        let b = b;

        RgbF {
            r: r,
            g: g,
            b: b,
        }
    }
}

impl ColorFormat for RgbF {

    fn channel_count() -> u8 {
        3
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::RGBf
    }

    fn to_bytes(self) -> Vec<u8> {
        let r = f32::to_ne_bytes(self.r);
        let g = f32::to_ne_bytes(self.g);
        let b = f32::to_ne_bytes(self.b);

        vec![r[0], r[1], r[2], r[3],
             g[0], g[1], g[2], g[3],
             b[0], b[1], b[2], b[3]]
    }

    fn as_bytes(&self) -> Vec<u8> {
        let r = f32::to_ne_bytes(self.r);
        let g = f32::to_ne_bytes(self.g);
        let b = f32::to_ne_bytes(self.b);

        vec![r[0], r[1], r[2], r[3],
             g[0], g[1], g[2], g[3],
             b[0], b[1], b[2], b[3]]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 12 {
            let r_f = f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let g_f = f32::from_ne_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            let b_f = f32::from_ne_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
            Ok(RgbF::new(r_f, g_f, b_f))
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::RGBf, self.to_bytes())
    }
}

impl From<Gray16> for RgbF {
    fn from(g16: Gray16) -> RgbF {
        let lum = g16.luminance as f32 / (u16::MAX as f32);
        RgbF::new(lum, lum, lum)
    }
}

impl From<GrayF> for RgbF {
    fn from(gf: GrayF) -> RgbF {
        let lum = gf.luminance;
        RgbF::new(lum, lum, lum)
    }
}

impl From<Rgb> for RgbF {
    fn from(rgb: Rgb) -> RgbF {
        let r_f = (rgb.r as f32) / (u8::MAX as f32);
        let g_f = (rgb.g as f32) / (u8::MAX as f32);
        let b_f = (rgb.b as f32) / (u8::MAX as f32);

        RgbF::new(r_f, g_f, b_f)
    }
}

impl From<Rgba> for RgbF {
    fn from(rgba: Rgba) -> RgbF {
        let r_f = (rgba.r as f32) / (u8::MAX as f32);
        let g_f = (rgba.g as f32) / (u8::MAX as f32);
        let b_f = (rgba.b as f32) / (u8::MAX as f32);

        RgbF::new(r_f, g_f, b_f)
    }
}

impl From<Srgb> for RgbF {
    fn from(srgb: Srgb) -> RgbF {
        let sr_f = (srgb.r as f32) / (u8::MAX as f32);
        let sg_f = (srgb.g as f32) / (u8::MAX as f32);
        let sb_f = (srgb.b as f32) / (u8::MAX as f32);

        let r_f = srgb_gamma_expand(sr_f);
        let g_f = srgb_gamma_expand(sg_f);
        let b_f = srgb_gamma_expand(sb_f);

        RgbF::new(r_f, g_f, b_f)
    }
}

impl From<SrgbF> for RgbF {
    fn from(srgb_f: SrgbF) -> RgbF {
        let r_f = srgb_gamma_expand(srgb_f.r);
        let g_f = srgb_gamma_expand(srgb_f.g);
        let b_f = srgb_gamma_expand(srgb_f.b);

        RgbF::new(r_f, g_f, b_f)
    }
}

impl From<Rgb48> for RgbF {
    fn from(rgb48: Rgb48) -> RgbF {
        let r_f = rgb48.r as f32 / (u16::MAX as f32);
        let g_f = rgb48.g as f32 / (u16::MAX as f32);
        let b_f = rgb48.b as f32 / (u16::MAX as f32);

        RgbF::new(r_f, g_f, b_f)
    }
}

impl From<Rgba64> for RgbF {
    fn from(rgba64: Rgba64) -> RgbF {
        let r_f = rgba64.r as f32 / (u16::MAX as f32);
        let g_f = rgba64.g as f32 / (u16::MAX as f32);
        let b_f = rgba64.b as f32 / (u16::MAX as f32);

        RgbF::new(r_f, g_f, b_f)
    }
}

impl From<RgbaF> for RgbF {
    fn from(rgbaf: RgbaF) -> RgbF {
        RgbF::new(rgbaf.r, rgbaf.g, rgbaf.b)
    }
}

impl From<Hsv> for RgbF {
    fn from(hsv: Hsv) -> RgbF {
        let c = hsv.s * hsv.v;
        let h = hsv.h / 60.0f32;
        let x = c * (1.0f32 - ((h % 2.0f32) - 1.0f32).abs());

        let (r, g, b) =
            if 0.0f32 <= h && h <= 1.0f32 {
                (c, x, 0.0f32)
            } else if 1.0f32 < h && h <= 2.0f32 {
                (x, c, 0.0f32)
            } else if 2.0f32 < h && h <= 3.0f32 {
                (0.0f32, c, x)
            } else if 3.0f32 < h && h <= 4.0f32 {
                (0.0f32, x, c)
            } else if 4.0f32 < h && h <= 5.0f32 {
                (x, 0.0f32, c)
            } else if 5.0f32 < h && h <= 6.0f32 {
                (c, 0.0f32, x)
            } else {
                (0.0f32, 0.0f32, 0.0f32)
            };

        let m = hsv.v - c;

        RgbF::new(r + m, g + m, b + m)
    }
}

impl From<Hsl> for RgbF {
    fn from(hsl: Hsl) -> RgbF {
        let c = (1.0f32 - ((2.0f32 * hsl.l) - 1.0f32).abs()) * hsl.s;
        let h = hsl.h / 60.0f32;
        let x = c * (1.0f32 - ((h % 2.0f32) - 1.0f32).abs());

        let (r, g, b) =
            if 0.0f32 <= h && h < 1.0f32 {
                (c, x, 0.0f32)
            } else if 1.0f32 <= h && h < 2.0f32 {
                (x, c, 0.0f32)
            } else if 2.0f32 <= h && h < 3.0f32 {
                (0.0f32, c, x)
            } else if 3.0f32 <= h && h < 4.0f32 {
                (0.0f32, x, c)
            } else if 4.0f32 <= h && h < 5.0f32 {
                (x, 0.0f32, c)
            } else if 5.0f32 <= h && h < 6.0f32 {
                (c, 0.0f32, x)
            } else {
                (0.0f32, 0.0f32, 0.0f32)
            };

        let m = hsl.l - (c / 2.0f32);
        RgbF::new(r + m, g + m, b + m)
    }
}

impl From<CieXyz> for RgbF {
    fn from(xyz: CieXyz) -> RgbF {
        let dest_ref_x = 0.95047f32;
        let dest_ref_y = 1.0f32;
        let dest_ref_z = 1.08883f32;

        let dest_ref_xyz = CieXyz::new(dest_ref_x, dest_ref_y, dest_ref_z);

        let src_ref_xyz =  if let Some(triplet) = xyz.ref_xyz {
            CieXyz::new(triplet.0, triplet.1, triplet.2)
        } else {
            dest_ref_xyz
        };

        //Don't need to do chromatic adaptation unless the reference white is different
        let xyz = if src_ref_xyz.x != dest_ref_xyz.x ||
                     src_ref_xyz.y != dest_ref_xyz.y ||
                     src_ref_xyz.z != dest_ref_xyz.z
        {
            CieXyz::chromatic_adaptation(xyz.clone(), dest_ref_xyz)
        } else {
            xyz
        };

        let xyz = [xyz.x, xyz.y, xyz.z];

        let m_inverse = [[3.2404542f32, -1.5371385f32, -0.4985314f32],
                         [-0.9692660f32, 1.8760108f32, 0.0415560f32],
                         [0.0556434f32, -0.2040259f32, 1.0572252f32]];

        let mut rgb = [0.0f32, 0.0f32, 0.0f32];

        for i in 0..rgb.len() {
            let mut row_sum = 0.0f32;
            for j in 0..xyz.len() {
                row_sum += m_inverse[i][j] * xyz[j];
            }
            rgb[i] += row_sum;
        }

        let r_f = rgb[0];
        let g_f = rgb[1];
        let b_f = rgb[2];

        RgbF::new(r_f, g_f, b_f)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct SrgbF {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl SrgbF {
    pub fn new(r: f32, g: f32, b: f32) -> SrgbF {
        let r = r;
        let g = g;
        let b = b;

        SrgbF {
            r: r,
            g: g,
            b: b,
        }
    }
}

impl ColorFormat for SrgbF {
    fn channel_count() -> u8 {
        3
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::SRGBf
    }

    fn to_bytes(self) -> Vec<u8> {
        let r = f32::to_ne_bytes(self.r);
        let g = f32::to_ne_bytes(self.g);
        let b = f32::to_ne_bytes(self.b);

        vec![r[0], r[1], r[2], r[3],
             g[0], g[1], g[2], g[3],
             b[0], b[1], b[2], b[3]]
    }

    fn as_bytes(&self) -> Vec<u8> {
        let r = f32::to_ne_bytes(self.r);
        let g = f32::to_ne_bytes(self.g);
        let b = f32::to_ne_bytes(self.b);

        vec![r[0], r[1], r[2], r[3],
             g[0], g[1], g[2], g[3],
             b[0], b[1], b[2], b[3]]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 12 {
            let r_f = f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let g_f = f32::from_ne_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            let b_f = f32::from_ne_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
            Ok(SrgbF::new(r_f, g_f, b_f))
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::SRGBf, self.to_bytes())
    }
}

impl From<GrayF> for SrgbF {
    fn from(gf: GrayF) -> SrgbF {
        let g = srgb_gamma_compress(gf.luminance);

        SrgbF::new(g, g, g)
    }
}

impl From<Rgb> for SrgbF {
    fn from(rgb: Rgb) -> SrgbF {
        let (r_f, g_f, b_f) = (rgb.r as f32, rgb.g as f32, rgb.b as f32);
        let (r_f, g_f, b_f) = ((r_f / (u8::MAX as f32)), (g_f / (u8::MAX as f32)), (b_f / (u8::MAX as f32)));

        let sr_f = srgb_gamma_compress(r_f);
        let sg_f = srgb_gamma_compress(g_f);
        let sb_f = srgb_gamma_compress(b_f);

        SrgbF::new(sr_f, sg_f, sb_f)
    }
}

impl From<Srgb> for SrgbF {
    fn from(srgb: Srgb) -> SrgbF {
        let sr_f = (srgb.r as f32) / (u8::MAX as f32);
        let sg_f = (srgb.g as f32) / (u8::MAX as f32);
        let sb_f = (srgb.b as f32) / (u8::MAX as f32);

        SrgbF::new(sr_f, sg_f, sb_f)
    }
}

impl From<RgbF> for SrgbF {
    fn from(rgbf: RgbF) -> SrgbF {
        let sr_f = srgb_gamma_compress(rgbf.r);
        let sg_f = srgb_gamma_compress(rgbf.g);
        let sb_f = srgb_gamma_compress(rgbf.b);

        SrgbF::new(sr_f, sg_f, sb_f)
    }
}

impl From<RgbaF> for SrgbF {
    fn from(rgbaf: RgbaF) -> SrgbF {
        let sr_f = srgb_gamma_compress(rgbaf.r);
        let sg_f = srgb_gamma_compress(rgbaf.g);
        let sb_f = srgb_gamma_compress(rgbaf.b);

        SrgbF::new(sr_f, sg_f, sb_f)
    }
}

impl From<Hsv> for SrgbF {
    fn from(hsv: Hsv) -> SrgbF {
        let c = hsv.s * hsv.v;
        let h = hsv.h / 60.0f32;
        let x = c * (1.0f32 - ((h % 2.0f32) - 1.0f32).abs());

        let (r, g, b) =
            if 0.0f32 <= h && h <= 1.0f32 {
                (c, x, 0.0f32)
            } else if 1.0f32 < h && h <= 2.0f32 {
                (x, c, 0.0f32)
            } else if 2.0f32 < h && h <= 3.0f32 {
                (0.0f32, c, x)
            } else if 3.0f32 < h && h <= 4.0f32 {
                (0.0f32, x, c)
            } else if 4.0f32 < h && h <= 5.0f32 {
                (x, 0.0f32, c)
            } else if 5.0f32 < h && h <= 6.0f32 {
                (c, 0.0f32, x)
            } else {
                (0.0f32, 0.0f32, 0.0f32)
            };

        let m = hsv.v - c;
        let (r_l, g_l, b_l) = (r + m, g + m, b + m);

        let sr_f = srgb_gamma_compress(r_l);
        let sg_f = srgb_gamma_compress(g_l);
        let sb_f = srgb_gamma_compress(b_l);

        SrgbF::new(sr_f, sg_f, sb_f)
    }
}

impl From<Hsl> for SrgbF {
    fn from(hsl: Hsl) -> SrgbF {
        let c = (1.0f32 - ((2.0f32 * hsl.l) - 1.0f32).abs()) * hsl.s;
        let h = hsl.h / 60.0f32;
        let x = c * (1.0f32 - ((h % 2.0f32) - 1.0f32).abs());

        let (r, g, b) =
            if 0.0f32 <= h && h < 1.0f32 {
                (c, x, 0.0f32)
            } else if 1.0f32 <= h && h < 2.0f32 {
                (x, c, 0.0f32)
            } else if 2.0f32 <= h && h < 3.0f32 {
                (0.0f32, c, x)
            } else if 3.0f32 <= h && h < 4.0f32 {
                (0.0f32, x, c)
            } else if 4.0f32 <= h && h < 5.0f32 {
                (x, 0.0f32, c)
            } else if 5.0f32 <= h && h < 6.0f32 {
                (c, 0.0f32, x)
            } else {
                (0.0f32, 0.0f32, 0.0f32)
            };

        let m = hsl.l - (c / 2.0f32);
        let (r_l, g_l, b_l) = (r + m, g + m, b + m);

        let sr_f = srgb_gamma_compress(r_l);
        let sg_f = srgb_gamma_compress(g_l);
        let sb_f = srgb_gamma_compress(b_l);

        SrgbF::new(sr_f, sg_f, sb_f)
    }
}

impl From<CieXyz> for SrgbF {
    fn from(xyz: CieXyz) -> SrgbF {

        //sRGB reference white (D65)
        let dest_ref_x = 0.95047f32;
        let dest_ref_y = 1.0f32;
        let dest_ref_z = 1.08883f32;

        let dest_ref_xyz = CieXyz::new(dest_ref_x, dest_ref_y, dest_ref_z);

        let src_ref_xyz =  if let Some(triplet) = xyz.ref_xyz {
            CieXyz::new(triplet.0, triplet.1, triplet.2)
        } else {
            dest_ref_xyz
        };

        //Don't need to do chromatic adaptation unless the reference white is different
        let xyz = if src_ref_xyz.x != dest_ref_xyz.x ||
                     src_ref_xyz.y != dest_ref_xyz.y ||
                     src_ref_xyz.z != dest_ref_xyz.z
        {
            CieXyz::chromatic_adaptation(xyz.clone(), dest_ref_xyz)
        } else {
            xyz
        };

        let xyz = [xyz.x, xyz.y, xyz.z];

        let m_inverse = [[3.2404542f32, -1.5371385f32, -0.4985314f32],
                         [-0.9692660f32, 1.8760108f32, 0.0415560f32],
                         [0.0556434f32, -0.2040259f32, 1.0572252f32]];

        let mut rgb = [0.0f32, 0.0f32, 0.0f32];

        for i in 0..rgb.len() {
            let mut row_sum = 0.0f32;
            for j in 0..xyz.len() {
                row_sum += m_inverse[i][j] * xyz[j];
            }
            rgb[i] += row_sum;
        }

        for i in 0..rgb.len() {
            rgb[i] = srgb_gamma_compress(rgb[i]);
        }

        let sr = rgb[0];
        let sg = rgb[1];
        let sb = rgb[2];

        SrgbF::new(sr, sg, sb)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

impl ColorFormat for Rgba {

    fn channel_count() -> u8 {
        4
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::RGBA
    }

    fn to_bytes(self) -> Vec<u8> {
        vec![self.r, self.g, self.b, self.a]
    }

    fn as_bytes(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b, self.a]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 4 {
            let rgb = Rgba::new(bytes[0], bytes[1], bytes[2], bytes[3]);
            Ok(rgb)
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::RGBA, self.to_bytes())
    }
}

impl From<Gray8> for Rgba {
    fn from(g8: Gray8) -> Rgba {
        Rgba::new(g8.luminance, g8.luminance, g8.luminance, 255)
    }
}

impl From<GrayF> for Rgba {
    fn from(gf: GrayF) -> Rgba {
        let lum = (gf.luminance * (u8::MAX as f32)) as u8;
        Rgba::new(lum, lum, lum, 255)
    }
}

impl From<Rgb> for Rgba {
    fn from(rgb: Rgb) -> Rgba {
        Rgba::new(rgb.r, rgb.g, rgb.b, 255)
    }
}

impl From<RgbF> for Rgba {
    fn from(rgbf: RgbF) -> Rgba {
        let r8 = (rgbf.r.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;
        let g8 = (rgbf.g.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;
        let b8 = (rgbf.b.clamp(0.0f32, 1.0f32) * (u8::MAX as f32)) as u8;

        Rgba::new(r8, g8, b8, 255)
    }
}

impl From<Rgba64> for Rgba {
    fn from(rgba_w: Rgba64) -> Rgba {
        let r_f = (rgba_w.r as f32) / (u16::MAX as f32);
        let g_f = (rgba_w.g as f32) / (u16::MAX as f32);
        let b_f = (rgba_w.b as f32) / (u16::MAX as f32);
        let a_f = (rgba_w.a as f32) / (u16::MAX as f32);

        let r8 = (r_f * (u8::MAX as f32)) as u8;
        let g8 = (g_f * (u8::MAX as f32)) as u8;
        let b8 = (b_f * (u8::MAX as f32)) as u8;
        let a8 = (a_f * (u8::MAX as f32)) as u8;

        Rgba::new(r8, g8, b8, a8)
    }
}

impl From<RgbaF> for Rgba {
    fn from(rgbaf: RgbaF) -> Rgba {
        let r8 = (rgbaf.r * (u8::MAX as f32)) as u8;
        let g8 = (rgbaf.g * (u8::MAX as f32)) as u8;
        let b8 = (rgbaf.b * (u8::MAX as f32)) as u8;
        let a8 = (rgbaf.a * (u8::MAX as f32)) as u8;

        Rgba::new(r8, g8, b8, a8)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Rgba64 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

impl Rgba64 {
    pub fn new(r: u16, g: u16, b: u16, a: u16) -> Rgba64 {
        Rgba64 {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

impl ColorFormat for Rgba64 {

    fn channel_count() -> u8 {
        4
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::RGBA64
    }

    fn to_bytes(self) -> Vec<u8> {
        let r16 = u16::to_ne_bytes(self.r);
        let g16 = u16::to_ne_bytes(self.g);
        let b16 = u16::to_ne_bytes(self.b);
        let a16 = u16::to_ne_bytes(self.a);

        vec![r16[0], r16[1],
             g16[0], g16[1],
             b16[0], b16[1],
             a16[0], a16[1]]
    }

    fn as_bytes(&self) -> Vec<u8> {
        let r16 = u16::to_ne_bytes(self.r);
        let g16 = u16::to_ne_bytes(self.g);
        let b16 = u16::to_ne_bytes(self.b);
        let a16 = u16::to_ne_bytes(self.a);

        vec![r16[0], r16[1],
             g16[0], g16[1],
             b16[0], b16[1],
             a16[0], a16[1]]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 8 {
            let r16 = u16::from_ne_bytes([bytes[0], bytes[1]]);
            let g16 = u16::from_ne_bytes([bytes[2], bytes[3]]);
            let b16 = u16::from_ne_bytes([bytes[4], bytes[5]]);
            let a16 = u16::from_ne_bytes([bytes[6], bytes[7]]);
            Ok(Rgba64::new(r16, g16, b16, a16))
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::RGBA64, self.to_bytes())
    }
}

impl From<Gray16> for Rgba64 {
    fn from(g16: Gray16) -> Rgba64 {
        let lum = g16.luminance;
        Rgba64::new(lum, lum, lum, 65535u16)
    }
}

impl From<GrayF> for Rgba64 {
    fn from(gf: GrayF) -> Rgba64 {
        let g16 = (gf.luminance * 65536.0f32) as u16;
        Rgba64::new(g16, g16, g16, 65535u16)
    }
}

impl From<Rgb> for Rgba64 {
    fn from(rgb: Rgb) -> Rgba64 {
        let r_f = rgb.r as f32 * ((u16::MAX as f32) / (u8::MAX as f32));
        let g_f = rgb.g as f32 * ((u16::MAX as f32) / (u8::MAX as f32));
        let b_f = rgb.b as f32 * ((u16::MAX as f32) / (u8::MAX as f32));

        let r16 = r_f as u16;
        let g16 = g_f as u16;
        let b16 = b_f as u16;

        Rgba64::new(r16, g16, b16, 65535u16)
    }
}

impl From<Rgb48> for Rgba64 {
    fn from(rgb_w: Rgb48) -> Rgba64 {
        Rgba64::new(rgb_w.r, rgb_w.g, rgb_w.b, 65535u16)
    }
}

impl From<RgbF> for Rgba64 {
    fn from(rgbf: RgbF) -> Rgba64 {
        let r16 = (rgbf.r.clamp(0.0f32, 1.0f32) * (u16::MAX as f32)) as u16;
        let g16 = (rgbf.g.clamp(0.0f32, 1.0f32) * (u16::MAX as f32)) as u16;
        let b16 = (rgbf.b.clamp(0.0f32, 1.0f32) * (u16::MAX as f32)) as u16;

        Rgba64::new(r16, g16, b16, 65535u16)
    }
}

impl From<Rgba> for Rgba64 {
    fn from(rgba: Rgba) -> Rgba64 {
        let r_f = rgba.r as f32 * ((u16::MAX as f32) / (u8::MAX as f32));
        let g_f = rgba.g as f32 * ((u16::MAX as f32) / (u8::MAX as f32));
        let b_f = rgba.b as f32 * ((u16::MAX as f32) / (u8::MAX as f32));
        let a_f = rgba.a as f32 * ((u16::MAX as f32) / (u8::MAX as f32));

        let r16 = r_f as u16;
        let g16 = g_f as u16;
        let b16 = b_f as u16;
        let a16 = a_f as u16;

        Rgba64::new(r16, g16, b16, a16)
    }
}

impl From<RgbaF> for Rgba64 {
    fn from(rgbaf: RgbaF) -> Rgba64 {
        let r16 = (rgbaf.r * (u16::MAX as f32)) as u16;
        let g16 = (rgbaf.g * (u16::MAX as f32)) as u16;
        let b16 = (rgbaf.b * (u16::MAX as f32)) as u16;
        let a16 = (rgbaf.a * (u16::MAX as f32)) as u16;

        Rgba64::new(r16, g16, b16, a16)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct RgbaF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl RgbaF {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> RgbaF {
        let r = r.clamp(0.0f32, 1.0f32);
        let g = g.clamp(0.0f32, 1.0f32);
        let b = b.clamp(0.0f32, 1.0f32);
        let a = a.clamp(0.0f32, 1.0f32);

        RgbaF {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

impl ColorFormat for RgbaF {

    fn channel_count() -> u8 {
        4
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::RGBAf
    }

    fn to_bytes(self) -> Vec<u8> {
        let r = f32::to_ne_bytes(self.r);
        let g = f32::to_ne_bytes(self.g);
        let b = f32::to_ne_bytes(self.b);
        let a = f32::to_ne_bytes(self.a);

        vec![r[0], r[1], r[2], r[3],
             g[0], g[1], g[2], g[3],
             b[0], b[1], b[2], b[3],
             a[0], a[1], a[2], a[3]]
    }

    fn as_bytes(&self) -> Vec<u8> {
        let r = f32::to_ne_bytes(self.r);
        let g = f32::to_ne_bytes(self.g);
        let b = f32::to_ne_bytes(self.b);
        let a = f32::to_ne_bytes(self.a);

        vec![r[0], r[1], r[2], r[3],
             g[0], g[1], g[2], g[3],
             b[0], b[1], b[2], b[3],
             a[0], a[1], a[2], a[3]]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 16 {
            let r_f = f32::from_ne_bytes([bytes[0],  bytes[1],  bytes[2],  bytes[3]]);
            let g_f = f32::from_ne_bytes([bytes[4],  bytes[5],  bytes[6],  bytes[7]]);
            let b_f = f32::from_ne_bytes([bytes[8],  bytes[9],  bytes[10], bytes[11]]);
            let a_f = f32::from_ne_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
            Ok(RgbaF::new(r_f, g_f, b_f, a_f))
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::RGBAf, self.to_bytes())
    }
}

impl From<Gray16> for RgbaF {
    fn from(g16: Gray16) -> RgbaF {
        let lum = g16.luminance as f32 / (u16::MAX as f32);
        RgbaF::new(lum, lum, lum, 1.0f32)
    }
}

impl From<GrayF> for RgbaF {
    fn from(gf: GrayF) -> RgbaF {
        let lum = gf.luminance;
        RgbaF::new(lum, lum, lum, 1.0f32)
    }
}

impl From<Rgb> for RgbaF {
    fn from(rgb: Rgb) -> RgbaF {
        let r_f = rgb.r as f32 / (u8::MAX as f32);
        let g_f = rgb.g as f32 / (u8::MAX as f32);
        let b_f = rgb.b as f32 / (u8::MAX as f32);

        RgbaF::new(r_f, g_f, b_f, 1.0f32)
    }
}

impl From<RgbF> for RgbaF {
    fn from(rgbf: RgbF) -> RgbaF {
        RgbaF::new(rgbf.r, rgbf.g, rgbf.b, 1.0f32)
    }
}

impl From<Rgba> for RgbaF {
    fn from(rgba: Rgba) -> RgbaF {
        let r_f = rgba.r as f32 / (u8::MAX as f32);
        let g_f = rgba.g as f32 / (u8::MAX as f32);
        let b_f = rgba.b as f32 / (u8::MAX as f32);
        let a_f = rgba.a as f32 / (u8::MAX as f32);

        RgbaF::new(r_f, g_f, b_f, a_f)
    }
}

impl From<Rgba64> for RgbaF {
    fn from(rgba_w: Rgba64) -> RgbaF {
        let r_f = rgba_w.r as f32 / (u16::MAX as f32);
        let g_f = rgba_w.g as f32 / (u16::MAX as f32);
        let b_f = rgba_w.b as f32 / (u16::MAX as f32);
        let a_f = rgba_w.a as f32 / (u16::MAX as f32);

        RgbaF::new(r_f, g_f, b_f, a_f)
    }
}
