use image::{DynamicImage, imageops, ImageFormat, ImageEncoder};
use rustler::types::tuple::make_tuple;
use rustler::{Binary, NifResult, Env, Term, OwnedBinary, Encoder};
use libwebp_sys::WebPImageHint;
use webp::{Encoder as WebPEncoder, WebPConfig};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
const DEFAULT_QUALITY: f32 = 60.0;

mod atoms {
    rustler::atoms! {
        ok
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn _rotate_image<'a>(path: String, direction: String) -> NifResult<String> {
    // Open the image file
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Err 22 Failed to open path: {:?}", e);
            return Err(rustler::Error::BadArg);
        }
    };

    let mut buf_reader = BufReader::new(file);

    // Read the first few bytes to guess the image format
    let mut buffer = [0; 10];
    if buf_reader.read_exact(&mut buffer).is_err() {
        eprintln!("Err 32 Failed to read the file header for format detection.");
        return Err(rustler::Error::BadArg);
    }

    let format = match image::guess_format(&buffer) {
        Ok(format) => format,
        Err(e) => {
            eprintln!("Err 39 Failed to guess image format: {:?}", e);
            return Err(rustler::Error::BadArg);
        }
    };

    // Reset the reader to the beginning of the file
    if buf_reader.seek(SeekFrom::Start(0)).is_err() {
        eprintln!("Failed to seek to the beginning of the file.");
        return Err(rustler::Error::BadArg);
    }

    // Load the image from the reset buffer
    let img = match image::load(buf_reader, format) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Err 48 Failed to load image: {:?}", e);
            return Err(rustler::Error::BadArg);
        }
    };

    // Rotate the image by 90 degrees
    let rotated_img = match direction.as_str() {
        "right" => img.rotate90().into_rgb8(),
        "left" => img.rotate270().into_rgb8(),
        "flip" => img.rotate180().into_rgb8(), 
        _ => img.rotate90().into_rgb8(), // nothing given, just rotate90
    };
    

    // Encode the rotated image back into the original format
    let mut encoded = Vec::new();
    match format {
        ImageFormat::Jpeg => {
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut encoded, 85);
            if let Err(e) = encoder.encode(&rotated_img, rotated_img.width(), rotated_img.height(), image::ColorType::Rgb8.into()) {
                eprintln!("Err 62 Failed to encode JPEG image: {:?}", e);
                return Err(rustler::Error::Term(Box::new(e.to_string())));
            }
        }
        ImageFormat::Png => {
            let encoder = image::codecs::png::PngEncoder::new(&mut encoded);
            let raw_image_data = rotated_img.as_raw();
            if let Err(e) = encoder.write_image(
                raw_image_data,
                rotated_img.width(),
                rotated_img.height(),
                image::ExtendedColorType::Rgb8,
            ) {
                eprintln!("Err 75 Failed to encode PNG image: {:?}", e);
                return Err(rustler::Error::Term(Box::new(e.to_string())));
            }
        }
        _ => {
            eprintln!("Err 80 Unsupported image format");
            return Err(rustler::Error::BadArg);
        }
    }

    // Write the encoded image data back to the file
    if let Err(e) = std::fs::write(&path, encoded) {
        eprintln!("Err 87 Failed to write image file: {:?}", e);
        return Err(rustler::Error::Term(Box::new(e.to_string())));
    }

    // Return the path of the overwritten image
    Ok(path)
}







// thumbnail creation functions
#[rustler::nif(schedule = "DirtyCpu")]
fn _create_thumbnail<'a>(
    env: Env<'a>,
    body: Binary<'a>,
    width: u32,
    height: u32,
    quality: Option<f32>,
    target_size: Option<u32>,
) -> NifResult<Term<'a>> {
    let image: DynamicImage =
        image::load_from_memory(body.as_slice()).map_err(|e| err_str(e.to_string()))?;

    let (width, height) = calc_dimension(&image, width, height);

    // Create thumbnail and convert to DynamicImage
    let thumbnail = imageops::thumbnail(&image, width, height);
    let thumbnail = DynamicImage::ImageRgba8(thumbnail);

    // Create WebP encoder and encode image
    let encoder = WebPEncoder::from_image(&thumbnail).map_err(|e| err_str(e.to_string()))?;
    let webp_data = encoder
        .encode_advanced(&webp_config(quality, target_size)?)
        .map_err(|e| err_str(format!("{:?}", e)))?;

    // Create OwnedBinary from WebP data
    let mut binary = OwnedBinary::new(webp_data.len())
        .ok_or_else(|| err_str("failed to allocate binary".to_string()))?;
    binary.as_mut_slice().copy_from_slice(&webp_data);

    let ok = atoms::ok().encode(env);
    Ok(make_tuple(env, &[ok, binary.release(env).encode(env)]))
}

fn err_str(error: String) -> rustler::Error {
    rustler::Error::Term(Box::new(error))
}

fn webp_config(quality: Option<f32>, target_size: Option<u32>) -> NifResult<WebPConfig> {
    let mut config = WebPConfig::new().map_err(|_| err_str("failed to create WebP config".to_string()))?;

    config.method = 2;
    config.image_hint = WebPImageHint::WEBP_HINT_PHOTO;
    config.sns_strength = 70;
    config.filter_sharpness = 2;
    config.filter_strength = 25;

    if let Some(size) = target_size {
        config.target_size = size as i32;
        config.pass = 5; // max iteration count
    } else if let Some(quality) = quality {
        config.quality = quality;
    } else {
        config.quality = DEFAULT_QUALITY;
    }

    Ok(config)
}

fn calc_dimension(image: &DynamicImage, width: u32, height: u32) -> (u32, u32) {
    if image.width() >= image.height() {
        // landscape
        let ratio = image.height() as f32 / image.width() as f32;
        let height = (ratio * width as f32).round() as u32;
        (width, height)
    } else {
        // portrait
        let ratio = image.width() as f32 / image.height() as f32;
        let width = (ratio * height as f32).round() as u32;
        (width, height)
    }
}

rustler::init!("Elixir.ImageTools");
