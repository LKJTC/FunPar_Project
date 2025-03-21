use image::io::Reader as ImageReader;
use image::codecs::jpeg::JpegEncoder;


fn open(source: &Path) -> image::DynamicImage {
    ImageReader::open(source)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
}
