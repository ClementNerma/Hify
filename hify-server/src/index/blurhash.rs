/// Heavily inspired by https://github.com/Raincal/blurhash-rs
use std::borrow::Cow;

use anyhow::{bail, Result};
use image::{DynamicImage, EncodableLayout};

// Maximum components to produce for blurhash
pub static MAX_BLURHASH_COMPONENTS_X: u32 = 3;
pub static MAX_BLURHASH_COMPONENTS_Y: u32 = 3;

/// Calculates the blurhash for the provided image
pub fn generate_blurhash(
    img: &DynamicImage,
    components_x: u32,
    components_y: u32,
) -> Result<String> {
    // Get the image as RGB bytes
    let bytes = match img {
        // RGB-8 image
        DynamicImage::ImageRgb8(img) => Cow::Borrowed(img.as_bytes()),

        // Non-RGB-8 image (e.g.: RGB8 + alpha, RGB16, etc.)
        _ => Cow::Owned(
            img.to_rgb8()
                .as_bytes()
                // TODO: optimize?
                .to_vec(),
        ),
    };

    encode(
        components_x,
        components_y,
        img.width(),
        img.height(),
        bytes.as_ref(),
    )
}

/// Calculates the blurhash for an image using the given x and y component counts.
fn encode(
    components_x: u32,
    components_y: u32,
    width: u32,
    height: u32,
    rgb_image: &[u8],
) -> Result<String> {
    if !(1..=9).contains(&components_x) || !(1..=9).contains(&components_y) {
        bail!("Components out of range");
    }

    let mut factors: Vec<[f32; 3]> = Vec::new();

    for y in 0..components_y {
        for x in 0..components_x {
            let factor = multiply_basis_function(x, y, width, height, rgb_image);
            factors.push(factor);
        }
    }

    let dc = factors[0];
    let ac = &factors[1..];

    let mut blurhash = String::new();

    let size_flag = (components_x - 1) + (components_y - 1) * 9;
    blurhash.push_str(&base83_encode(size_flag, 1));

    let maximum_value: f32;
    if !ac.is_empty() {
        let mut actualmaximum_value = 0.0;
        for i in 0..components_y * components_x - 1 {
            actualmaximum_value = f32::max(ac[i as usize][0], actualmaximum_value);
            actualmaximum_value = f32::max(ac[i as usize][1], actualmaximum_value);
            actualmaximum_value = f32::max(ac[i as usize][2], actualmaximum_value);
        }

        let quantised_maximum_value = f32::max(
            0.,
            f32::min(82., f32::floor(actualmaximum_value * 166. - 0.5)),
        ) as u32;

        maximum_value = (quantised_maximum_value + 1) as f32 / 166.;
        blurhash.push_str(&base83_encode(quantised_maximum_value, 1));
    } else {
        maximum_value = 1.;
        blurhash.push_str(&base83_encode(0, 1));
    }

    blurhash.push_str(&base83_encode(dc_encode(dc), 4));

    for i in 0..components_y * components_x - 1 {
        blurhash.push_str(&base83_encode(ac_encode(ac[i as usize], maximum_value), 2));
    }

    Ok(blurhash)
}

fn multiply_basis_function(
    component_x: u32,
    component_y: u32,
    width: u32,
    height: u32,
    rgb: &[u8],
) -> [f32; 3] {
    use std::f32::consts::PI;

    let mut r = 0.;
    let mut g = 0.;
    let mut b = 0.;
    let normalisation = match (component_x, component_y) {
        (0, 0) => 1.,
        _ => 2.,
    };

    let bytes_per_row = width * 3;

    for y in 0..height {
        for x in 0..width {
            let basis = f32::cos(PI * component_x as f32 * x as f32 / width as f32)
                * f32::cos(PI * component_y as f32 * y as f32 / height as f32);
            r += basis * srgb_to_linear(u32::from(rgb[(3 * x + y * bytes_per_row) as usize]));
            g += basis * srgb_to_linear(u32::from(rgb[(3 * x + 1 + y * bytes_per_row) as usize]));
            b += basis * srgb_to_linear(u32::from(rgb[(3 * x + 2 + y * bytes_per_row) as usize]));
        }
    }

    let scale = normalisation / (width * height) as f32;

    [r * scale, g * scale, b * scale]
}

fn ac_encode(value: [f32; 3], maximum_value: f32) -> u32 {
    let quant_r = i32::max(
        0,
        i32::min(
            18,
            f32::floor(sign_pow(value[0] / maximum_value, 0.5) * 9. + 9.5) as i32,
        ),
    );
    let quant_g = i32::max(
        0,
        i32::min(
            18,
            f32::floor(sign_pow(value[1] / maximum_value, 0.5) * 9. + 9.5) as i32,
        ),
    );
    let quant_b = i32::max(
        0,
        i32::min(
            18,
            f32::floor(sign_pow(value[2] / maximum_value, 0.5) * 9. + 9.5) as i32,
        ),
    );

    (quant_r * 19 * 19 + quant_g * 19 + quant_b) as u32
}

fn dc_encode(value: [f32; 3]) -> u32 {
    let rounded_r = linear_to_srgb(value[0]);
    let rounded_g = linear_to_srgb(value[1]);
    let rounded_b = linear_to_srgb(value[2]);
    (rounded_r << 16) + (rounded_g << 8) + rounded_b
}

/// linear 0.0-1.0 floating point to srgb 0-255 integer conversion.
fn linear_to_srgb(value: f32) -> u32 {
    let v = f32::max(0., f32::min(1., value));
    if v <= 0.003_130_8 {
        (v * 12.92 * 255. + 0.5).round() as u32
    } else {
        ((1.055 * f32::powf(v, 1. / 2.4) - 0.055) * 255. + 0.5).round() as u32
    }
}

/// srgb 0-255 integer to linear 0.0-1.0 floating point conversion.
fn srgb_to_linear(value: u32) -> f32 {
    let v = value as f32 / 255.;
    if v <= 0.04045 {
        v / 12.92
    } else {
        f32::powf((v + 0.055) / 1.055, 2.4)
    }
}

static BASE83_CHARACTERS: [u8; 83] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
    b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l',
    b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'#', b'$',
    b'%', b'*', b'+', b',', b'-', b'.', b':', b';', b'=', b'?', b'@', b'[', b']', b'^', b'_', b'{',
    b'|', b'}', b'~',
];

fn base83_encode(value: u32, length: u32) -> String {
    let mut result = String::new();

    for i in 1..=length {
        let digit: u32 = (value / u32::pow(83, length - i)) % 83;
        result.push(BASE83_CHARACTERS[digit as usize] as char);
    }

    result
}

fn sign_pow(val: f32, exp: f32) -> f32 {
    sign(val) * f32::powf(val.abs(), exp)
}

fn sign(n: f32) -> f32 {
    if n < 0. {
        -1.
    } else {
        1.
    }
}
