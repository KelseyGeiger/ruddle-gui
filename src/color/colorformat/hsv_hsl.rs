use super::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl Hsv {
    pub fn new(h: f32, s: f32, v: f32) -> Hsv {
        Hsv {
            h: h,
            s: s,
            v: v,
        }
    }
}

impl ColorFormat for Hsv {

    fn channel_count() -> u8 {
        3
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::HSV
    }

    fn to_bytes(self) -> Vec<u8> {
        let h = f32::to_ne_bytes(self.h);
        let s = f32::to_ne_bytes(self.s);
        let v = f32::to_ne_bytes(self.v);

        vec![h[0], h[1], h[2], h[3],
             s[0], s[1], s[2], s[3],
             v[0], v[1], v[2], v[3]]
    }

    fn as_bytes(&self) -> Vec<u8> {
        let h = f32::to_ne_bytes(self.h);
        let s = f32::to_ne_bytes(self.s);
        let v = f32::to_ne_bytes(self.v);

        vec![h[0], h[1], h[2], h[3],
             s[0], s[1], s[2], s[3],
             v[0], v[1], v[2], v[3]]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 12 {
            let h = f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let s = f32::from_ne_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            let v = f32::from_ne_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
            Ok(Hsv::new(h, s, v))
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::HSV, self.to_bytes())
    }
}

impl From<Rgb> for Hsv {
    fn from(rgb: Rgb) -> Hsv {
        let r_f = rgb.r as f32 / (u8::MAX as f32);
        let g_f = rgb.g as f32 / (u8::MAX as f32);
        let b_f = rgb.b as f32 / (u8::MAX as f32);

        let channels = [r_f, g_f, b_f];

        let mut max_idx = 0;
        let mut min_idx = 0;
        for i in 0..channels.len() {
            if channels[i] >= channels[max_idx] {
                max_idx = i;
            }
            if channels[i] <= channels[min_idx] {
                min_idx = i;
            }
        }

        let c = channels[max_idx] - channels[min_idx];
        let v = channels[max_idx];

        let h = if c == 0.0f32 {
            0.0f32
        } else if max_idx == 0 {
            60.0f32 * ((g_f - b_f) / c)
        } else if max_idx == 1 {
            60.0f32 * (2.0f32 + ((b_f - r_f) / c))
        } else {
            60.0f32 * (4.0f32 + ((r_f - g_f) / c))
        };

        let s = if v == 0.0f32 {
            0.0f32
        } else {
            c / v
        };

        Hsv::new(h, s, v)
    }
}

impl From<Srgb> for Hsv {
    fn from(srgb: Srgb) -> Hsv {
        let r_f = srgb.r as f32 / (u8::MAX as f32);
        let g_f = srgb.g as f32 / (u8::MAX as f32);
        let b_f = srgb.b as f32 / (u8::MAX as f32);

        let r_f = srgb_gamma_expand(r_f);
        let g_f = srgb_gamma_expand(g_f);
        let b_f = srgb_gamma_expand(b_f);

        let channels = [r_f, g_f, b_f];

        let mut max_idx = 0;
        let mut min_idx = 0;
        for i in 0..channels.len() {
            if channels[i] >= channels[max_idx] {
                max_idx = i;
            }
            if channels[i] <= channels[min_idx] {
                min_idx = i;
            }
        }

        let c = channels[max_idx] - channels[min_idx];
        let v = channels[max_idx];

        let h = if c == 0.0f32 {
            0.0f32
        } else if max_idx == 0 {
            60.0f32 * ((g_f - b_f) / c)
        } else if max_idx == 1 {
            60.0f32 * (2.0f32 + ((b_f - r_f) / c))
        } else {
            60.0f32 * (4.0f32 + ((r_f - g_f) / c))
        };

        let s = if v == 0.0f32 {
            0.0f32
        } else {
            c / v
        };

        Hsv::new(h, s, v)
    }
}

impl From<SrgbF> for Hsv {
    fn from(srgb_f: SrgbF) -> Hsv {
        let r_f = srgb_gamma_expand(srgb_f.r);
        let g_f = srgb_gamma_expand(srgb_f.g);
        let b_f = srgb_gamma_expand(srgb_f.b);

        let channels = [r_f, g_f, b_f];

        let mut max_idx = 0;
        let mut min_idx = 0;
        for i in 0..channels.len() {
            if channels[i] >= channels[max_idx] {
                max_idx = i;
            }
            if channels[i] <= channels[min_idx] {
                min_idx = i;
            }
        }

        let c = channels[max_idx] - channels[min_idx];
        let v = channels[max_idx];

        let h = if c == 0.0f32 {
            0.0f32
        } else if max_idx == 0 {
            60.0f32 * ((g_f - b_f) / c)
        } else if max_idx == 1 {
            60.0f32 * (2.0f32 + ((b_f - r_f) / c))
        } else {
            60.0f32 * (4.0f32 + ((r_f - g_f) / c))
        };

        let s = if v == 0.0f32 {
            0.0f32
        } else {
            c / v
        };

        Hsv::new(h, s, v)
    }
}

impl From<RgbF> for Hsv {
    fn from(rgbf: RgbF) -> Hsv {
        let channels = [rgbf.r.clamp(0.0f32, 1.0f32),
                        rgbf.g.clamp(0.0f32, 1.0f32),
                        rgbf.b.clamp(0.0f32, 1.0f32)];

        let mut max_idx = 0;
        let mut min_idx = 0;
        for i in 0..channels.len() {
            if channels[i] >= channels[max_idx] {
                max_idx = i;
            }
            if channels[i] <= channels[min_idx] {
                min_idx = i;
            }
        }

        let c = channels[max_idx] - channels[min_idx];
        let v = channels[max_idx];

        let h = if c == 0.0f32 {
            0.0f32
        } else if max_idx == 0 {
            60.0f32 * ((rgbf.g - rgbf.b) / c)
        } else if max_idx == 1 {
            60.0f32 * (2.0f32 + ((rgbf.b - rgbf.r) / c))
        } else {
            60.0f32 * (4.0f32 + ((rgbf.r - rgbf.g) / c))
        };

        let s = if v == 0.0f32 {
            0.0f32
        } else {
            c / v
        };

        Hsv::new(h, s, v)
    }
}

impl From<RgbaF> for Hsv {
    fn from(rgbaf: RgbaF) -> Hsv {
        let channels = [rgbaf.r, rgbaf.g, rgbaf.b];

        let mut max_idx = 0;
        let mut min_idx = 0;
        for i in 0..channels.len() {
            if channels[i] >= channels[max_idx] {
                max_idx = i;
            }
            if channels[i] <= channels[min_idx] {
                min_idx = i;
            }
        }

        let c = channels[max_idx] - channels[min_idx];
        let v = channels[max_idx];

        let h = if c == 0.0f32 {
            0.0f32
        } else if max_idx == 0 {
            60.0f32 * ((rgbaf.g - rgbaf.b) / c)
        } else if max_idx == 1 {
            60.0f32 * (2.0f32 + ((rgbaf.b - rgbaf.r) / c))
        } else {
            60.0f32 * (4.0f32 + ((rgbaf.r - rgbaf.g) / c))
        };

        let s = if v == 0.0f32 {
            0.0f32
        } else {
            c / v
        };

        Hsv::new(h, s, v)
    }
}

impl From<Hsl> for Hsv {
    fn from(hsl: Hsl) -> Hsv {
        let v = hsl.l + hsl.s * (if hsl.l >= 0.5f32 { 1.0f32 - hsl.l } else { hsl.l });
        let s = if v == 0.0f32 {
            0.0f32
        } else {
            2.0f32 * (1.0f32 - (hsl.l / v))
        };

        Hsv::new(hsl.h, s, v)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl Hsl {
    pub fn new(h: f32, s: f32, l: f32) -> Hsl {
        Hsl {
            h: h,
            s: s,
            l: l,
        }
    }
}

impl ColorFormat for Hsl {

    fn channel_count() -> u8 {
        3
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::HSL
    }

    fn to_bytes(self) -> Vec<u8> {
        let h = f32::to_ne_bytes(self.h);
        let s = f32::to_ne_bytes(self.s);
        let l = f32::to_ne_bytes(self.l);

        vec![h[0], h[1], h[2], h[3],
             s[0], s[1], s[2], s[3],
             l[0], l[1], l[2], l[3]]
    }

    fn as_bytes(&self) -> Vec<u8> {
        let h = f32::to_ne_bytes(self.h);
        let s = f32::to_ne_bytes(self.s);
        let l = f32::to_ne_bytes(self.l);

        vec![h[0], h[1], h[2], h[3],
             s[0], s[1], s[2], s[3],
             l[0], l[1], l[2], l[3]]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 12 {
            let h = f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let s = f32::from_ne_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            let l = f32::from_ne_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
            Ok(Hsl::new(h, s, l))
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::HSL, self.to_bytes())
    }
}

impl From<Rgb> for Hsl {
    fn from(rgb: Rgb) -> Hsl {
        let r_f = (rgb.r as f32) / (u8::MAX as f32);
        let g_f = (rgb.g as f32) / (u8::MAX as f32);
        let b_f = (rgb.b as f32) / (u8::MAX as f32);

        let channels = [r_f, g_f, b_f];

        let mut max_idx = 0;
        let mut min_idx = 0;
        for i in 0..channels.len() {
            if channels[i] >= channels[max_idx] {
                max_idx = i;
            }
            if channels[i] <= channels[min_idx] {
                min_idx = i;
            }
        }

        let c = channels[max_idx] - channels[min_idx];
        let l = (channels[max_idx] + channels[min_idx]) / 2.0f32;

        let h = if c == 0.0f32 {
            0.0f32
        } else if max_idx == 0 {
            60.0f32 * ((g_f - b_f) / c)
        } else if max_idx == 1 {
            60.0f32 * (2.0f32 + ((b_f - r_f) / c))
        } else {
            60.0f32 * (4.0f32 + ((r_f - g_f) / c))
        };

        let s = (channels[max_idx] - l) / (if l >= 0.5f32 { 1.0f32 - l } else { l });

        Hsl::new(h, s, l)
    }
}

impl From<Srgb> for Hsl {
    fn from(srgb: Srgb) -> Hsl {
        let r_f = (srgb.r as f32) / (u8::MAX as f32);
        let g_f = (srgb.g as f32) / (u8::MAX as f32);
        let b_f = (srgb.b as f32) / (u8::MAX as f32);

        let r_f = srgb_gamma_expand(r_f);
        let g_f = srgb_gamma_expand(g_f);
        let b_f = srgb_gamma_expand(b_f);

        let channels = [r_f, g_f, b_f];

        let mut max_idx = 0;
        let mut min_idx = 0;
        for i in 0..channels.len() {
            if channels[i] >= channels[max_idx] {
                max_idx = i;
            }
            if channels[i] <= channels[min_idx] {
                min_idx = i;
            }
        }

        let c = channels[max_idx] - channels[min_idx];
        let l = (channels[max_idx] + channels[min_idx]) / 2.0f32;

        let h = if c == 0.0f32 {
            0.0f32
        } else if max_idx == 0 {
            60.0f32 * ((g_f - b_f) / c)
        } else if max_idx == 1 {
            60.0f32 * (2.0f32 + ((b_f - r_f) / c))
        } else {
            60.0f32 * (4.0f32 + ((r_f - g_f) / c))
        };

        let s = (channels[max_idx] - l) / (if l >= 0.5f32 { 1.0f32 - l } else { l });

        Hsl::new(h, s, l)
    }
}

impl From<SrgbF> for Hsl {
    fn from(srgb_f: SrgbF) -> Hsl {
        let r_f = srgb_gamma_expand(srgb_f.r);
        let g_f = srgb_gamma_expand(srgb_f.g);
        let b_f = srgb_gamma_expand(srgb_f.b);

        let channels = [r_f, g_f, b_f];

        let mut max_idx = 0;
        let mut min_idx = 0;
        for i in 0..channels.len() {
            if channels[i] >= channels[max_idx] {
                max_idx = i;
            }
            if channels[i] <= channels[min_idx] {
                min_idx = i;
            }
        }

        let c = channels[max_idx] - channels[min_idx];
        let l = (channels[max_idx] + channels[min_idx]) / 2.0f32;

        let h = if c == 0.0f32 {
            0.0f32
        } else if max_idx == 0 {
            60.0f32 * ((g_f - b_f) / c)
        } else if max_idx == 1 {
            60.0f32 * (2.0f32 + ((b_f - r_f) / c))
        } else {
            60.0f32 * (4.0f32 + ((r_f - g_f) / c))
        };

        let s = (channels[max_idx] - l) / (if l >= 0.5f32 { 1.0f32 - l } else { l });

        Hsl::new(h, s, l)
    }
}

impl From<RgbF> for Hsl {
    fn from(rgbf: RgbF) -> Hsl {
        let channels = [rgbf.r.clamp(0.0f32, 1.0f32),
                        rgbf.g.clamp(0.0f32, 1.0f32),
                        rgbf.b.clamp(0.0f32, 1.0f32)];

        let mut max_idx = 0;
        let mut min_idx = 0;
        for i in 0..channels.len() {
            if channels[i] >= channels[max_idx] {
                max_idx = i;
            }
            if channels[i] <= channels[min_idx] {
                min_idx = i;
            }
        }

        let c = channels[max_idx] - channels[min_idx];
        let l = (channels[max_idx] + channels[min_idx]) / 2.0f32;

        let h = if c == 0.0f32 {
            0.0f32
        } else if max_idx == 0 {
            60.0f32 * ((rgbf.g - rgbf.b) / c)
        } else if max_idx == 1 {
            60.0f32 * (2.0f32 + ((rgbf.b - rgbf.r) / c))
        } else {
            60.0f32 * (4.0f32 + ((rgbf.r - rgbf.g) / c))
        };

        let s = (channels[max_idx] - l) / (if l >= 0.5f32 { 1.0f32 - l } else { l });

        Hsl::new(h, s, l)
    }
}

impl From<RgbaF> for Hsl {
    fn from(rgbaf: RgbaF) -> Hsl {
        let channels = [rgbaf.r, rgbaf.g, rgbaf.b];

        let mut max_idx = 0;
        let mut min_idx = 0;
        for i in 0..channels.len() {
            if channels[i] >= channels[max_idx] {
                max_idx = i;
            }
            if channels[i] <= channels[min_idx] {
                min_idx = i;
            }
        }

        let c = channels[max_idx] - channels[min_idx];
        let l = (channels[max_idx] + channels[min_idx]) / 2.0f32;

        let h = if c == 0.0f32 {
            0.0f32
        } else if max_idx == 0 {
            60.0f32 * ((rgbaf.g - rgbaf.b) / c)
        } else if max_idx == 1 {
            60.0f32 * (2.0f32 + ((rgbaf.b - rgbaf.r) / c))
        } else {
            60.0f32 * (4.0f32 + ((rgbaf.r - rgbaf.g) / c))
        };

        let s = (channels[max_idx] - l) / (if l >= 0.5f32 { 1.0f32 - l } else { l });

        Hsl::new(h, s, l)
    }
}

impl From<Hsv> for Hsl {
    fn from(hsv: Hsv) -> Hsl {
        let l = hsv.v * (1.0f32 - (hsv.s / 2.0f32));
        let s = if l == 1.0f32 || l == 0.0f32 {
            0.0f32
        } else {
            (hsv.v - l) / (if l >= 0.5f32 { 1.0f32 - l } else { l })
        };

        Hsl::new(hsv.h, s, l)
    }
}
