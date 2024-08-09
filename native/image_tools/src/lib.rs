use image::{load_from_memory, Rgba, DynamicImage, imageops};
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};
use rustler::types::tuple::make_tuple;
use rustler::{Binary, NifResult, Env, Term, OwnedBinary, Encoder};
use libwebp_sys::WebPImageHint;
use webp::{Encoder as WebPEncoder, WebPConfig};

const DEFAULT_QUALITY: f32 = 60.0;

mod atoms {
    rustler::atoms! {
        ok
    }
}

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

fn rotate_image<'a>(env: Env<'a>, binary: Binary<'a>, degrees: f32) -> NifResult<(Binary<'a>, u32, u32)> {
    // Load the image from the binary data
    let img = load_from_memory(&binary).map_err(|_| rustler::Error::BadArg)?;

    // Convert to RGBA if necessary
    let img = img.to_rgba8();

    // Rotate the image using imageproc
    let rotated_img = rotate_about_center(
        &img,
        degrees.to_radians(),
        Interpolation::Bilinear,
        Rgba([0u8, 0u8, 0u8, 0u8]),
    );

    // Convert the rotated image back to a binary format
    let mut buf = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new(&mut buf);
    encoder.encode_image(&DynamicImage::ImageRgba8(rotated_img.clone())).map_err(|e| err_str(e.to_string()))?;

    // Create OwnedBinary from encoded image
    let mut binary = OwnedBinary::new(buf.len()).ok_or_else(|| rustler::Error::Term(Box::new("Failed to create binary")))?;
    binary.as_mut_slice().copy_from_slice(&buf);

    Ok((binary.release(env), rotated_img.width(), rotated_img.height()))
}

rustler::init!("Elixir.ImageTools");
