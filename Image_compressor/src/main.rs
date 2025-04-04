use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use image::ImageReader;
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
    let img = open(source);
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

// # project_folder
// # ├── dest_dir 
// # │   ├── picture1.jpg -> this will come after compress file from soruce_dir
// # │   ├── picture2.jpg -> this will come after compress file from soruce_dir
// # ├── source_dir
// # │   ├── picture1.png
// # │   ├── picture2.gif
// # ├── src
// # │   ├── main.rs
// # ├── Cargo.lock
// # ├── Cargo.toml

/// Main function that sets up paths and initiates the compression process.
fn main() {

    let source = PathBuf::from("source_dir");
    let destination = PathBuf::from("dest_dir");
    let quality = 80;
    folder(&source, &destination, quality);

}