use std::fs::File;
use std::path::{Path, PathBuf};
use image::io::Reader as ImageReader;
use image::codecs::jpeg::JpegEncoder;

fn open(source: &Path) -> image::DynamicImage {
    ImageReader::open(source)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
}

fn saveimage(img: &image::DynamicImage, destination: &Path, quality: u8) {
    let mut output_file = File::create(destination)
        .expect("Failed, try again");
    let mut encoder = JpegEncoder::new_with_quality(&mut output_file, quality);
    encoder.encode_image(img)
        .expect("Failed, try again");
}

fn compress(source: &Path, destination: &Path, quality: u8) {
    let img = open_image(source);
    saveimage(&img, destination, quality);
}

fn folder(source: &Path, dest: &Path, quality: u8) {
    fs::create_dir_all(dest).expect("Failed try again");
    let entries: Vec<_> = fs::read_dir(source)
        .expect("Failed try aga")
        .map(|entry| entry.expect("Failed try aga").path())
        .collect();
    
    entries.par_iter().for_each(|path| {
        if path.is_file() {
            let mut dest_path = PathBuf::from(dest);
            if let Some(filename) = path.file_name() {
                dest_path.push(filename);
                dest_path.set_extension("jpg"); 
                compress(path, &dest_path, quality);
            }
        }
    });
    println!("Reached here");
}