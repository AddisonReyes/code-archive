use image::{
    DynamicImage, GenericImageView, ImageFormat,
    imageops::{FilterType, crop_imm},
    open,
};

pub fn resize_image(path: &str, width: u32, height: u32) -> DynamicImage {
    let img = open(path).expect("Failure to load/open image");
    img.resize(width, height, FilterType::Lanczos3)
}

pub fn save_image(img: &DynamicImage, output_path: &str, format: ImageFormat) {
    img.save_with_format(output_path, format)
        .expect("Failed to save image.");
    println!("Image saved: {output_path} ({format:?})");
}

pub fn rotate_image(path: &str, degress: u32) -> DynamicImage {
    let img = open(path).expect("Failure to load/open image");
    match degress {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => {
            eprintln!("Unsopported rotation angle. Please give 90, 180, 270.");
            img
        }
    }
}

pub fn resize_image_maintain_ratio(
    path: &str,
    new_width: Option<u32>,
    new_height: Option<u32>,
) -> DynamicImage {
    let img = open(path).expect("Failure to load/open image");
    let (width, height) = img.dimensions();

    let ratio = width as f32 / height as f32;
    let (resize_width, resize_height) = match (new_width, new_height) {
        (Some(w), Some(h)) => (w, h),
        (Some(w), None) => (w, (w as f32 / ratio).round() as u32),
        (None, Some(h)) => ((h as f32 * ratio).round() as u32, h),
        (None, None) => (width, height),
    };

    img.resize(resize_width, resize_height, FilterType::Lanczos3)
}

pub fn crop_image(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    let cropped_img = crop_imm(img, x, y, width, height);
    DynamicImage::ImageRgba8(cropped_img.to_image())
}
