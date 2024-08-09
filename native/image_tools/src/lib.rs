use rustler::{Env, NifResult, Binary, OwnedBinary};
use image::{load_from_memory, Rgba, RgbaImage, DynamicImage};
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};

#[rustler::nif]
fn rotate_image<'a>(env: Env<'a>, binary: Binary<'a>, degrees: f32) -> NifResult<(Binary<'a>, u32, u32)> {
    // Load the image from the binary data
    let img = load_from_memory(&binary).map_err(|_| rustler::Error::BadArg)?;

    // Convert to RGBA if necessary
    let img = img.to_rgba8();

    // Rotate the image using imageproc
    let rotated_img: RgbaImage = rotate_about_center(
        &img,
        degrees.to_radians(),
        Interpolation::Bilinear,
        Rgba([0u8, 0u8, 0u8, 0u8]),
    );

    // Convert the rotated image back to a binary format
    let mut buf = Vec::new();
    {
        // Using a block to limit the scope of encoder to avoid moving `buf`
        let mut encoder = image::codecs::jpeg::JpegEncoder::new(&mut buf);
        encoder.encode_image(&DynamicImage::ImageRgba8(rotated_img.clone())).unwrap();
    }

    // Convert Vec<u8> to OwnedBinary and return
    let mut binary = OwnedBinary::new(buf.len()).ok_or(rustler::Error::Term(Box::new("Failed to create binary")))?;
    binary.as_mut_slice().copy_from_slice(&buf);

    Ok((binary.release(env), rotated_img.width(), rotated_img.height()))
}

rustler::init!("Elixir.ImageTools");