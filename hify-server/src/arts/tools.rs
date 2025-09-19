use std::{borrow::Cow, fs, path::Path};

use anyhow::{Context, Result};
use image::{GenericImage, RgbImage, imageops::FilterType};

pub fn resize_image(image: &RgbImage, nwidth: u32, nheight: u32) -> RgbImage {
    image::imageops::resize(image, nwidth, nheight, FilterType::Lanczos3)
}

pub fn resize_image_constraint<'a>(
    image: &'a RgbImage,
    smallest_side_px: u32,
) -> Cow<'a, RgbImage> {
    if image.width() <= smallest_side_px && image.height() <= smallest_side_px {
        return Cow::Borrowed(image);
    }

    let diviser = if image.width() < image.height() {
        image.height() as f64 / smallest_side_px as f64
    } else {
        image.width() as f64 / smallest_side_px as f64
    };

    Cow::Owned(resize_image(
        image,
        (image.width() as f64 / diviser) as u32,
        (image.height() as f64 / diviser) as u32,
    ))
}

pub fn assemble_four_images(
    top_left: &RgbImage,
    top_right: &RgbImage,
    bottom_left: &RgbImage,
    bottom_right: &RgbImage,
    side_px: u32,
) -> Result<RgbImage> {
    assert!(side_px.is_multiple_of(2));

    // TODO: choose how to handle images with different aspect ratios
    let mut image = RgbImage::new(side_px, side_px);

    let resize = |image| resize_image(image, side_px / 2, side_px / 2);

    image.copy_from(&resize(top_left), 0, 0)?;
    image.copy_from(&resize(top_right), side_px / 2, 0)?;
    image.copy_from(&resize(bottom_left), 0, side_px / 2)?;
    image.copy_from(&resize(bottom_right), side_px / 2, side_px / 2)?;

    Ok(image)
}

pub fn save_image_webp(image: &RgbImage, path: &Path) -> Result<()> {
    let encoder = webp::Encoder::from_rgb(image, image.width(), image.height()).encode(70f32);

    fs::write(path, &*encoder)
        .with_context(|| format!("Failed to write image to file: {}", path.display()))?;

    Ok(())
}
