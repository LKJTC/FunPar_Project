use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use image::io::Reader as ImageReader;
use image::codecs::jpeg::JpegEncoder;

/// Opens and decodes an image from the given source path.
fn open_image(source: &Path) -> image::DynamicImage {
    ImageReader::open(source)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
}
