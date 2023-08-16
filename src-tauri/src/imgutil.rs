use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use image::{io::Reader, DynamicImage, GenericImageView};

pub fn open_image(path: PathBuf) -> Result<DynamicImage, Box<dyn Error>> {
    Ok(Reader::open(path)?.decode()?)
}

pub fn save_image(image: &DynamicImage, path: PathBuf) -> Result<(), Box<dyn Error>> {
    save_bytes(image.as_bytes(), path)
}

pub fn save_bytes(bytes: &[u8], path: PathBuf) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(path.parent().unwrap())?;
    let mut file = File::create(path)?;
    Ok(file.write_all(bytes)?)
}

pub fn resize(image: &DynamicImage, width: u32) -> Result<DynamicImage, Box<dyn Error>> {
    let (w, h) = image.dimensions();
    Ok(image.resize(
        width,
        (h * width) / w,
        image::imageops::FilterType::Lanczos3,
    ))
}
