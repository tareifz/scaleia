#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate image;

use std::fs::File;

use docopt::Docopt;
use image::GenericImage;

#[derive(Debug, Deserialize)]
struct Args {
    arg_directory: String,
    arg_width: Option<u32>,
    arg_height: Option<u32>,
}

const USAGE: &'static str = "
Scaleia.

Usage:
  scaleia <directory> <width> <height>

Options:
  -h --help     Show this screen.
";

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());
    
    let width = match args.arg_width {
        Some(t) => t,
        None => panic!("This can not happen."),
    };

    let height = match args.arg_height {
        Some(t) => t,
        None => panic!("This can not happen."),
    };

    let directory = std::path::Path::new(&args.arg_directory);

    if !directory.exists() {
        panic!("This directory does not exists!");
    }

    // if we reached this point thats mean we have valid values and valid
    // directory path. Note: if the directory's path starts with "/" we will
    // get an error, so this should be fixed.

    // println!("{:?}", directory);

    for entry in directory.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {

            println!("{:?}", entry.path());
            // here we didn't check if entry is an image, directory or unsupported
            // file type, we let the Image library to try to open the file, if there
            // is an error we will ignore it and process the next file.
            let img = image::open(&entry.path());
            if let Ok(img) = img {
                println!("dimensions before scaling: {:?}", img.dimensions());

                let new_width = (img.width() * width) / 100;
                let new_height = (img.height() * height) / 100;
                if (width > 100) || (height > 100)
                {
                    
                }
                let img = img.resize_exact(new_width, new_height, image::FilterType::CatmullRom);

                println!("dimensions after scaling: {:?}", img.dimensions());

                let ref mut fout = File::create(&entry.path()).unwrap();
                let mut img_format: image::ImageFormat = image::JPEG;

                if let Some(ext) = entry.path().extension() {
                    if let Some(ext) = ext.to_str() {
                        match ext.to_uppercase().as_str() {
                            "PNG" => img_format = image::ImageFormat::PNG,
                            "GIF" => img_format = image::ImageFormat::GIF,
                            "JPEG" => img_format = image::ImageFormat::JPEG,
                            "BMP" => img_format = image::ImageFormat::BMP,
                            "WEBP" => img_format = image::ImageFormat::WEBP,
                            "PPM" => img_format = image::ImageFormat::PPM,
                            "TIFF" => img_format = image::ImageFormat::TIFF,
                            "TGA" => img_format = image::ImageFormat::TGA,
                            "ICO" => img_format = image::ImageFormat::ICO,
                            "HDR" => img_format = image::ImageFormat::HDR,
                            &_ => img_format = image::ImageFormat::JPEG,
                        }
                    }
                };

                img.save(fout, img_format).unwrap()
                
            }
            
        }
    }
}
