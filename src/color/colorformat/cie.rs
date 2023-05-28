use super::*;

//A note on scale: Y = 1.0 is maximum luminance. Some systems use Y = 100 for this- they need to be scaled
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CieXyz {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub ref_xyz: Option<(f32, f32, f32)>,
}

/*  The default value for CieXYZ should be the same white point sRGB uses-
 *  what else will we use for colors on a computer?
 *  This is defined as (x = 0.3127, y = 0.3290) for the D65 CIE standard illuminant, in the xyY color space
 *  In XYZ, this corresponds to (X = 0.95047, Y = 1.0, Z = 1.0883)
 */
impl Default for CieXyz {
    fn default() -> CieXyz {
        CieXyz {
            x: 0.95047f32,
            y: 1.0f32,
            z: 1.08883f32,
            ref_xyz: None,
        }
    }
}

#[allow(dead_code)]
impl CieXyz {
    pub fn new(x: f32, y: f32, z: f32) -> CieXyz {
        CieXyz {
            x,
            y,
            z,
            ref_xyz: None
        }
    }

    pub fn with_reference_white(mut self, ref_white: CieXyz) -> CieXyz {
        match ref_white.ref_xyz {
            None => {
                self.ref_xyz = Some((ref_white.x, ref_white.y, ref_white.z));
            },
            Some(triplet) => {
                let def = CieXyz::default();

                /*  To keep ourselves tethered here, if we're going to use a reference white defined in
                 *  terms of *another* reference white, let's arbitrarily suggest convertiing to the default
                 *  reference white- the one used by sRGB- before taking that new reference white
                 */
                if triplet.0 == def.x && triplet.1 == def.y &&triplet.2 == def.z {
                    self.ref_xyz = Some((ref_white.x, ref_white.y, ref_white.z));
                } else {
                    let ref_white = CieXyz::chromatic_adaptation(ref_white, def);
                    self.ref_xyz = Some((ref_white.x, ref_white.y, ref_white.z));
                }
            }
        }

        self
    }

    pub fn chromatic_adaptation(xyz: CieXyz, dest_ref_xyz: CieXyz) -> CieXyz {
        let src_ref_xyz = if let Some(ref_white) = xyz.ref_xyz {
            [ref_white.0, ref_white.1, ref_white.2]
        } else {
            [0.95047f32, 1.0f32, 1.08883f32]    //sRGB's reference white (D65)
        };

        let dest_ref_xyz = [dest_ref_xyz.x, dest_ref_xyz.y, dest_ref_xyz.z];

        //Bradford XYZ scaling matrices- apparently used by Adobe Photoshop?
        let m_a = [[0.8951000f32, 0.2664000f32, -0.1614000f32],
                   [-0.7502000f32, 1.7135000f32, 0.0367000f32],
                   [0.0389000f32, -0.0685000f32, 1.0296000f32]];

        let m_a_inverse = [[0.9869929f32, -0.1470543f32, 0.1599627f32],
                           [0.4323053f32, 0.5183603f32, 0.0492912f32],
                           [-0.0085287f32, 0.0400428f32, 0.9684867f32]];

        let mut src_rho_gamma_beta = [0.0f32, 0.0f32, 0.0f32];
        let mut dest_rho_gamma_beta = [0.0f32, 0.0f32, 0.0f32];

        for i in 0..src_rho_gamma_beta.len() {
            let mut row_sum = 0.0f32;
            for j in 0..m_a.len() {
                row_sum += m_a[i][j] * src_ref_xyz[j];
            }
            src_rho_gamma_beta[i] += row_sum;
        }

        for i in 0..dest_rho_gamma_beta.len() {
            let mut row_sum = 0.0f32;
            for j in 0..m_a.len() {
                row_sum += m_a[i][j] * dest_ref_xyz[j];
            }
            dest_rho_gamma_beta[i] += row_sum;
        }

        let (src_rho, src_gamma, src_beta) = (src_rho_gamma_beta[0], src_rho_gamma_beta[1], src_rho_gamma_beta[2]);
        let (dest_rho, dest_gamma, dest_beta) = (dest_rho_gamma_beta[0], dest_rho_gamma_beta[1], dest_rho_gamma_beta[2]);

        let adapt_mat = [[(dest_rho / src_rho), 0.0f32, 0.0f32],
                         [0.0f32, (dest_gamma / src_gamma), 0.0f32],
                         [0.0f32, 0.0f32, (dest_beta / src_beta)]];

        let mut m_ad = [[0.0f32, 0.0f32, 0.0f32],
                        [0.0f32, 0.0f32, 0.0f32],
                        [0.0f32, 0.0f32, 0.0f32]];

        for i in 0..m_ad.len() {
            for j in 0..adapt_mat.len() {
                let mut sum = 0.0f32;
                for k in 0..m_a.len() {
                    sum += adapt_mat[i][k] * m_a[k][j];
                }
                m_ad[i][j] = sum;
            }
        }

        let mut m = [[0.0f32, 0.0f32, 0.0f32],
                     [0.0f32, 0.0f32, 0.0f32],
                     [0.0f32, 0.0f32, 0.0f32]];

        for i in 0..m.len() {
            for j in 0..m_a_inverse.len() {
                let mut sum = 0.0f32;
                for k in 0..m_ad.len() {
                    sum += m_a_inverse[i][k] * m_ad[k][j];
                }
                m[i][j] = sum;
            }
        }

        let mut dest_xyz = [0.0f32, 0.0f32, 0.0f32];

        let xyz = [xyz.x, xyz.y, xyz.z];

        for i in 0..dest_xyz.len() {
            let mut row_sum = 0.0f32;
            for j in 0..m.len() {
                row_sum += m[i][j] * xyz[j];
            }
            dest_xyz[i] += row_sum;
        }

        CieXyz {
            x: dest_xyz[0],
            y: dest_xyz[1],
            z: dest_xyz[2],
            ref_xyz: Some((dest_xyz[0], dest_xyz[1], dest_xyz[2]))
        }
    }

    pub fn adjust_to_reference_white(&mut self, dest_ref_white: CieXyz) {
        match dest_ref_white.ref_xyz {
            None => {
                *self = CieXyz::chromatic_adaptation(*self, dest_ref_white);
            },
            Some(triplet) => {
                let def = CieXyz::default();

                /*  To keep ourselves tethered here, if we're going to use a reference white defined in
                 *  terms of *another* reference white, let's arbitrarily suggest convertiing to the default
                 *  reference white- the one used by sRGB- before taking that new reference white
                 */
                if triplet.0 == def.x && triplet.1 == def.y &&triplet.2 == def.z {
                    *self = CieXyz::chromatic_adaptation(*self, dest_ref_white);
                } else {
                    let ref_white = CieXyz::chromatic_adaptation(dest_ref_white, def);
                    *self = CieXyz::chromatic_adaptation(*self, ref_white);
                }
            }
        }

    }
}

impl ColorFormat for CieXyz {

    fn channel_count() -> u8 {
        3
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::CIEXYZ
    }

    fn to_bytes(self) -> Vec<u8> {
        let x = f32::to_ne_bytes(self.x);
        let y = f32::to_ne_bytes(self.y);
        let z = f32::to_ne_bytes(self.z);

        let mut bytes = vec![x[0], x[1], x[2], x[3],
                             y[0], y[1], y[2], y[3],
                             z[0], z[1], z[2], z[3]];

        if let Some(triple) = self.ref_xyz {
            let ref_x = f32::to_ne_bytes(triple.0);
            let ref_z = f32::to_ne_bytes(triple.1);
            let ref_y = f32::to_ne_bytes(triple.2);

            bytes.extend_from_slice(&ref_x);
            bytes.extend_from_slice(&ref_y);
            bytes.extend_from_slice(&ref_z);
        }

        bytes
    }

    fn as_bytes(&self) -> Vec<u8> {
        let x = f32::to_ne_bytes(self.x);
        let y = f32::to_ne_bytes(self.y);
        let z = f32::to_ne_bytes(self.z);

        let mut bytes = vec![x[0], x[1], x[2], x[3],
                             y[0], y[1], y[2], y[3],
                             z[0], z[1], z[2], z[3]];

        if let Some(triple) = self.ref_xyz {
            let ref_x = f32::to_ne_bytes(triple.0);
            let ref_z = f32::to_ne_bytes(triple.1);
            let ref_y = f32::to_ne_bytes(triple.2);

            bytes.extend_from_slice(&ref_x);
            bytes.extend_from_slice(&ref_y);
            bytes.extend_from_slice(&ref_z);
        }

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 12 {
            let x = f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let y = f32::from_ne_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            let z = f32::from_ne_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

            Ok(CieXyz::new(x, y, z))
        } else if bytes.len() == 24 {
            let x = f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let y = f32::from_ne_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            let z = f32::from_ne_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

            let ref_x = f32::from_ne_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
            let ref_y = f32::from_ne_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
            let ref_z = f32::from_ne_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);

            Ok(CieXyz::new(x, y, z).with_reference_white(CieXyz::new(ref_x, ref_y, ref_z)))
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly either {} or {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), Self::bytes_per_pixel() * 2, bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::CIEXYZ, self.to_bytes())
    }
}

impl From<Rgb> for CieXyz {
    fn from(rgb: Rgb) -> CieXyz {
        let r_f = rgb.r as f32 / (u8::MAX as f32);
        let g_f = rgb.g as f32 / (u8::MAX as f32);
        let b_f = rgb.b as f32 / (u8::MAX as f32);

        let rgb = [r_f, g_f, b_f];

        let m = [[0.4124564f32, 0.3575761f32, 0.1804375f32],
                 [0.2126729f32, 0.7151522f32, 0.0721750f32],
                 [0.0193339f32, 0.1191920f32, 0.9503041f32]];

        let mut xyz = [0.0f32, 0.0f32, 0.0f32];

        for i in 0..xyz.len() {
            let mut row_sum = 0.0f32;
            for j in 0..rgb.len() {
                row_sum += m[i][j] * rgb[j];
            }
            xyz[i] += row_sum;
        }

        //Assuming that this is linearized sRGB, which is pretty likely for most purposes
        let ref_xyz = CieXyz::new(0.95047f32, 1.0f32, 1.08883f32);
        CieXyz::new(xyz[0], xyz[1], xyz[2]).with_reference_white(ref_xyz)
    }
}

impl From<Srgb> for CieXyz {
    fn from(srgb: Srgb) -> CieXyz {
        let sr_f = srgb.r as f32 / (u8::MAX as f32);
        let sg_f = srgb.g as f32 / (u8::MAX as f32);
        let sb_f = srgb.b as f32 / (u8::MAX as f32);

        let r_f = srgb_gamma_expand(sr_f);
        let g_f = srgb_gamma_expand(sg_f);
        let b_f = srgb_gamma_expand(sb_f);

        let rgb = [r_f, g_f, b_f];

        let m = [[0.4124564f32, 0.3575761f32, 0.1804375f32],
                 [0.2126729f32, 0.7151522f32, 0.0721750f32],
                 [0.0193339f32, 0.1191920f32, 0.9503041f32]];

        let mut xyz = [0.0f32, 0.0f32, 0.0f32];

        for i in 0..xyz.len() {
            let mut row_sum = 0.0f32;
            for j in 0..rgb.len() {
                row_sum += m[i][j] * rgb[j];
            }
            xyz[i] += row_sum;
        }

        //Assuming that this is linearized sRGB, which is pretty likely for most purposes
        let ref_xyz = CieXyz::new(0.95047f32, 1.0f32, 1.08883f32);
        CieXyz::new(xyz[0], xyz[1], xyz[2]).with_reference_white(ref_xyz)
    }
}

impl From<SrgbF> for CieXyz {
    fn from(srgb_f: SrgbF) -> CieXyz {
        let r_f = srgb_gamma_expand(srgb_f.r);
        let g_f = srgb_gamma_expand(srgb_f.g);
        let b_f = srgb_gamma_expand(srgb_f.b);

        let rgb = [r_f, g_f, b_f];

        let m = [[0.4124564f32, 0.3575761f32, 0.1804375f32],
                 [0.2126729f32, 0.7151522f32, 0.0721750f32],
                 [0.0193339f32, 0.1191920f32, 0.9503041f32]];

        let mut xyz = [0.0f32, 0.0f32, 0.0f32];

        for i in 0..xyz.len() {
            let mut row_sum = 0.0f32;
            for j in 0..rgb.len() {
                row_sum += m[i][j] * rgb[j];
            }
            xyz[i] += row_sum;
        }

        //Assuming that this is linearized sRGB, which is pretty likely for most purposes
        let ref_xyz = CieXyz::new(0.95047f32, 1.0f32, 1.08883f32);
        CieXyz::new(xyz[0], xyz[1], xyz[2]).with_reference_white(ref_xyz)
    }
}

impl From<RgbF> for CieXyz {
    fn from(rgbf: RgbF) -> CieXyz {
        let rgb = [rgbf.r.clamp(0.0f32, 1.0f32),
                   rgbf.g.clamp(0.0f32, 1.0f32),
                   rgbf.b.clamp(0.0f32, 1.0f32)];

        let m = [[0.4124564f32, 0.3575761f32, 0.1804375f32],
                 [0.2126729f32, 0.7151522f32, 0.0721750f32],
                 [0.0193339f32, 0.1191920f32, 0.9503041f32]];

        let mut xyz = [0.0f32, 0.0f32, 0.0f32];

        for i in 0..xyz.len() {
            let mut row_sum = 0.0f32;
            for j in 0..rgb.len() {
                row_sum += m[i][j] * rgb[j];
            }
            xyz[i] += row_sum;
        }

        //Assuming that this is linearized sRGB, which is pretty likely for most purposes
        let ref_xyz = CieXyz::new(0.95047f32, 1.0f32, 1.08883f32);
        CieXyz::new(xyz[0], xyz[1], xyz[2]).with_reference_white(ref_xyz)
    }
}

impl From<RgbaF> for CieXyz {
    fn from(rgbaf: RgbaF) -> CieXyz {
        let rgb = [rgbaf.r, rgbaf.g, rgbaf.b];

        let m = [[0.4124564f32, 0.3575761f32, 0.1804375f32],
                 [0.2126729f32, 0.7151522f32, 0.0721750f32],
                 [0.0193339f32, 0.1191920f32, 0.9503041f32]];

        let mut xyz = [0.0f32, 0.0f32, 0.0f32];

        for i in 0..xyz.len() {
            let mut row_sum = 0.0f32;
            for j in 0..rgb.len() {
                row_sum += m[i][j] * rgb[j];
            }
            xyz[i] += row_sum;
        }

        //Assuming that this is linearized sRGB, which is pretty likely for most purposes
        let ref_xyz = CieXyz::new(0.95047f32, 1.0f32, 1.08883f32);
        CieXyz::new(xyz[0], xyz[1], xyz[2]).with_reference_white(ref_xyz)
    }
}

impl From<CieLab> for CieXyz {
    fn from(lab: CieLab) -> CieXyz {

        let ref_white = if let Some(ref_xyz) = lab.ref_xyz {
            CieXyz::new(ref_xyz.0, ref_xyz.1, ref_xyz.2)
        } else {
            CieXyz::default()
        };

        let epsilon = 0.008856f32;
        let kappa = 903.3f32;

        let f_y = (lab.l + 16.0f32) / 116.0f32;
        let f_z = f_y - (lab.b / 200.0f32);
        let f_x = (lab.a / 500.0f32) + f_y;

        let x_r = if f_x.powi(3) > epsilon {
            f_x.powi(3)
        } else {
            (116.0f32 * f_x - 16.0f32) / kappa
        };

        let y_r = if lab.l > kappa * epsilon {
            f_y.powi(3)
        } else {
            lab.l / kappa
        };

        let z_r = if f_z.powi(3) > epsilon {
            f_z.powi(3)
        } else {
            (116.0f32 * f_z - 16.0f32) / kappa
        };

        let x = x_r * ref_white.x;
        let y = y_r * ref_white.y;
        let z = z_r * ref_white.z;

        CieXyz::new(x, y, z).with_reference_white(ref_white)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CieLab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
    pub ref_xyz: Option<(f32, f32, f32)>,
}

#[allow(dead_code)]
impl CieLab {
    pub fn new(l: f32, a: f32, b: f32) -> CieLab {
        CieLab {
            l,
            a,
            b,
            ref_xyz: None,
        }
    }

    pub fn with_reference_white(mut self, ref_white: CieXyz) -> CieLab {
        match ref_white.ref_xyz {
            None => {
                self.ref_xyz = Some((ref_white.x, ref_white.y, ref_white.z));
            },
            Some(triplet) => {
                let def = CieXyz::default();

                if triplet.0 == def.x && triplet.1 == def.y &&triplet.2 == def.z {
                    self.ref_xyz = Some((ref_white.x, ref_white.y, ref_white.z));
                } else {
                    let ref_white = CieXyz::chromatic_adaptation(ref_white, def);
                    self.ref_xyz = Some((ref_white.x, ref_white.y, ref_white.z));
                }
            }
        }

        self
    }

    pub fn adapt_to_reference_white(lab: CieLab, ref_xyz: CieXyz) -> CieLab {
        let to_xyz: CieXyz = lab.into();
        let new_xyz = CieXyz::chromatic_adaptation(to_xyz, ref_xyz);
        new_xyz.into()
    }
}

impl ColorFormat for CieLab {

    fn channel_count() -> u8 {
        3
    }

    fn format_name() -> ColorFormatName {
        ColorFormatName::CIELab
    }

    fn to_bytes(self) -> Vec<u8> {
        let l = f32::to_ne_bytes(self.l);
        let a = f32::to_ne_bytes(self.a);
        let b = f32::to_ne_bytes(self.b);

        let mut bytes = vec![l[0], l[1], l[2], l[3],
                             a[0], a[1], a[2], a[3],
                             b[0], b[1], b[2], b[3]];

        if let Some(triple) = self.ref_xyz {
            let ref_x = f32::to_ne_bytes(triple.0);
            let ref_z = f32::to_ne_bytes(triple.1);
            let ref_y = f32::to_ne_bytes(triple.2);

            bytes.extend_from_slice(&ref_x);
            bytes.extend_from_slice(&ref_y);
            bytes.extend_from_slice(&ref_z);
        }

        bytes
    }

    fn as_bytes(&self) -> Vec<u8> {
        let l = f32::to_ne_bytes(self.l);
        let a = f32::to_ne_bytes(self.a);
        let b = f32::to_ne_bytes(self.b);

        let mut bytes = vec![l[0], l[1], l[2], l[3],
                             a[0], a[1], a[2], a[3],
                             b[0], b[1], b[2], b[3]];

        if let Some(triple) = self.ref_xyz {
            let ref_x = f32::to_ne_bytes(triple.0);
            let ref_z = f32::to_ne_bytes(triple.1);
            let ref_y = f32::to_ne_bytes(triple.2);

            bytes.extend_from_slice(&ref_x);
            bytes.extend_from_slice(&ref_y);
            bytes.extend_from_slice(&ref_z);
        }

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 12 {
            let l = f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let a = f32::from_ne_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            let b = f32::from_ne_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

            Ok(CieLab::new(l, a, b))
        } else if bytes.len() == 24 {
            let l = f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let a = f32::from_ne_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            let b = f32::from_ne_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

            let ref_x = f32::from_ne_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
            let ref_y = f32::from_ne_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
            let ref_z = f32::from_ne_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);

            Ok(CieLab::new(l, a, b).with_reference_white(CieXyz::new(ref_x, ref_y, ref_z)))
        } else {
            Err(format!("Tried to convert to {} with wrong number of bytes.\n\t \
                            Hint: Should be exactly either {} or {} byte(s). {} bytes were provided.",
                            type_name::<Self>(), Self::bytes_per_pixel(), Self::bytes_per_pixel() * 2, bytes.len())
                            .to_owned())
        }
    }

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>) {
        (ColorFormatName::CIELab, self.to_bytes())
    }
}

impl From<CieXyz> for CieLab {
    fn from(xyz: CieXyz) -> CieLab {
        let ref_white = if let Some(ref_xyz) = xyz.ref_xyz {
            CieXyz::new(ref_xyz.0, ref_xyz.1, ref_xyz.2)
        } else {
            CieXyz::default()
        };

        let x_r = xyz.x / ref_white.x;
        let y_r = xyz.y / ref_white.y;
        let z_r = xyz.z / ref_white.z;

        let epsilon = 0.008856f32;
        let kappa = 903.3f32;

        let f_x = if x_r > epsilon {
            x_r.cbrt()
        } else {
            (kappa * x_r + 16.0f32) / 116.0f32
        };

        let f_y = if y_r > epsilon {
            y_r.cbrt()
        } else {
            (kappa * y_r + 16.0f32) / 116.0f32
        };

        let f_z = if z_r > epsilon {
            z_r.cbrt()
        } else {
            (kappa * z_r + 16.0f32) / 116.0f32
        };

        let l = 116.0f32 * f_y - 16.0f32;
        let a = 500.0f32 * (f_x - f_y);
        let b = 200.0f32 * (f_y - f_z);

        CieLab::new(l, a, b).with_reference_white(ref_white)
    }
}
