use std::{error::Error, path::PathBuf, time::Instant};

use image::{DynamicImage, EncodableLayout};
use webp::{Encoder, WebPMemory};

use crate::imgutil;

fn to_webp(image: &DynamicImage, quality: f32) -> Result<Box<WebPMemory>, Box<dyn Error>> {
    let encoder = Encoder::from_image(image)?;
    Ok(Box::new(encoder.encode(quality)))
}

pub fn save_to_webp(
    image: &DynamicImage,
    path: PathBuf,
    quality: f32,
) -> Result<(u128, u128), Box<dyn Error>> {
    let start = Instant::now();
    let webp = to_webp(image, quality)?;
    let first = start.elapsed().as_millis();
    imgutil::save_bytes(webp.as_bytes(), path)?;
    let second = start.elapsed().as_millis() - first;
    Ok((first, second))
}
