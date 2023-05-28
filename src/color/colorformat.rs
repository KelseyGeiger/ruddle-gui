use std::vec::*;
use std::any::type_name;
use std::convert::{Into, From};

pub use self::grayscale::*;
pub use self::rgb::*;
pub use self::hsv_hsl::*;
pub use self::cie::*;

pub mod grayscale;
pub mod rgb;
pub mod hsv_hsl;
pub mod cie;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColorFormatName {
    Gray8, Gray16, Grayf,
    RGB, SRGB, RGB48,
    RGBA, RGBA64,
    RGBf, SRGBf, RGBAf,
    HSV, HSL,
    CIEXYZ, CIELab
}

pub trait ColorFormat: Sized {
    fn bytes_per_pixel() -> usize {
        std::mem::size_of::<Self>()
    }

    fn channel_count() -> u8;
    fn format_name() -> ColorFormatName;

    //The following methods **should use native byte order**.
    //These are for working on the same machine
    fn to_bytes(self) -> Vec<u8>;
    fn as_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, String>;

    fn to_raw_parts(self) -> (ColorFormatName, Vec<u8>);
}

#[allow(dead_code)]
pub fn get_bytes_per_pixel<C: ColorFormat>(_p: C) -> usize {
    C::bytes_per_pixel()
}

#[allow(dead_code)]
pub fn get_bpp_for_format(cfn: ColorFormatName) -> usize {
    match cfn {
        ColorFormatName::Gray8 => {
            Gray8::bytes_per_pixel()
        },
        ColorFormatName::Gray16 => {
            Gray16::bytes_per_pixel()
        },
        ColorFormatName::Grayf => {
            GrayF::bytes_per_pixel()
        },
        ColorFormatName::RGB => {
            Rgb::bytes_per_pixel()
        },
        ColorFormatName::SRGB => {
            Srgb::bytes_per_pixel()
        },
        ColorFormatName::RGB48 => {
            Rgb48::bytes_per_pixel()
        },
        ColorFormatName::RGBA => {
            Rgba::bytes_per_pixel()
        },
        ColorFormatName::RGBA64 => {
            Rgba64::bytes_per_pixel()
        },
        ColorFormatName::RGBf => {
            RgbF::bytes_per_pixel()
        },
        ColorFormatName::SRGBf => {
            SrgbF::bytes_per_pixel()
        },
        ColorFormatName::RGBAf => {
            RgbaF::bytes_per_pixel()
        },
        ColorFormatName::HSV => {
            Hsv::bytes_per_pixel()
        },
        ColorFormatName::HSL => {
            Hsl::bytes_per_pixel()
        },
        ColorFormatName::CIEXYZ => {
            CieXyz::bytes_per_pixel()
        },
        ColorFormatName::CIELab => {
            CieLab::bytes_per_pixel()
        }
    }
}

// Utiliity functions for conversions

fn gray_from_rgb(r: f32, g: f32, b: f32) -> f32 {
    (0.2126f32 * r) + (0.7152f32 * g) + (0.0722f32 * b)
}

//Linear RGB -> sRGB
fn srgb_gamma_compress(channel: f32) -> f32 {
    if channel <= 0.0031308f32 {
        12.92f32 * channel
    } else {
        1.055f32 * channel.powf(1.0f32 / 2.4f32) - 0.055f32
    }
}

//sRGB -> Linear RGB
fn srgb_gamma_expand(channel: f32) -> f32 {
    if channel <= 0.04045f32 {
        channel / 12.92f32
    } else {
        ((channel + 0.055)/1.055).powf(2.4)
    }
}
