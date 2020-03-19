use image::imageops::FilterType;
use image::GenericImageView;
use image::DynamicImage;
use image::ImageFormat;

use rayon::prelude::*;

use std::path::PathBuf;
use std::sync::Mutex;
use std::io;
use std::fs;





fn wait_for_exit() {
    println!("Press Enter to exit");
    match io::stdin().read_line(&mut String::new()) {
        Err(_) => {},
        Ok(_) => {},
    }
}

fn save_image(path: PathBuf, img: DynamicImage) {
    match img.save_with_format(&path, ImageFormat::Png) {
        Err(_) => println!("\t\tSaving errored for image at {}", path.to_str().unwrap()),
        Ok(_) => {},
    }
}

fn save_png(path: &PathBuf, img: DynamicImage, suffix: &str) {
    let path_file_name = path
        .file_stem()
        .unwrap();
    let path_string = format!(
        "{}{}",
        path_file_name
            .to_str()
            .unwrap(),
        suffix
    );
    save_image(
        PathBuf::from(&path)
            .with_file_name(path_string)
            .with_extension("png"),
        img
    );
}





fn process(path: &PathBuf) -> bool {
    match image::open(path) {
        Err(_) => {
            println!("\tFile is not an image");
        },
        Ok(img) => {
            // Make sure the image is a valid thumbnail
            if img.width() != (img.height() * 2) {
                println!("\tFile must be an image with an aspect ratio of 2:1");
                return false;
            }
            // Delete the old file
            match fs::remove_file(path) {
                Ok(_) => {},
                Err(_) => {
                    println!("\tCould not delete file");
                    return false;
                },
            }
            // Create the new images
            let mut img_resized = img.resize_exact(2048, 1024, FilterType::Lanczos3);
            let img_crop_left   = img_resized.crop(0, 0, 1024, 1024);
            let img_crop_right  = img_resized.crop(1024, 0, 1024, 1024);
            let img_small       = img.resize_exact(1024, 512, FilterType::Lanczos3);
            // Save the new files
            save_png(path, img_resized,    "");
            save_png(path, img_crop_left,  " - Left");
            save_png(path, img_crop_right, " - Right");
            save_png(path, img_small,      " - Small");
            // Return success
            return true;
        }
    }
    return false;
}





fn main() {
    let mut paths_files: Vec<PathBuf> = Vec::new();

    for path_string in std::env::args().skip(1) {
        let path = PathBuf::from(path_string);
        if path.is_file() {
            paths_files.push(path);
        }
    }

    if paths_files.len() <= 0 {
        println!("No files were provided");
        wait_for_exit();
        return;
    }

    println!("Creating thumbnails from {} files", paths_files.len());

    let paths_processed = Mutex::new(0);
    paths_files.par_iter().for_each(|path| {
        match path.file_name() {
            None => {},
            Some(path_file) => {
                let path_file_string = path_file.to_str().unwrap();
                println!("\tProcessing {}", path_file_string);
                if process(path) {
                    *paths_processed.lock().unwrap() += 1;
                }
                println!("\tProcessed {}", path_file_string);
            },
        }
    });

    println!("Created thumbnails for {} files", paths_processed.lock().unwrap());

    wait_for_exit();
}