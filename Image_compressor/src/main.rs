use std::path::{Path, PathBuf};
use image::io::Reader as ImageReader;

/// Opens and decodes an image from the given source path.
fn open_image(source: &Path) -> image::DynamicImage {
    ImageReader::open(source)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
}

