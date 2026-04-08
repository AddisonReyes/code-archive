use image::{DynamicImage, GenericImageView, ImageFormat, imageops::FilterType, open};

fn resize_image(path: &str, width: u32, height: u32) -> DynamicImage {
    let img = open(path).expect("Failure to load/open image");
    img.resize(width, height, FilterType::Lanczos3)
}

fn save_image(img: &DynamicImage, output_path: &str, format: ImageFormat) {
    img.save_with_format(output_path, format)
        .expect("Failed to save image.");
    println!("Image saved: {output_path} ({format:?})");
}

fn rotate_image(path: &str, degress: u32) -> DynamicImage {
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

fn main() {
    println!("Image processing");

    const IMG_PATH: &str = "assets/sample_img.jpg";
    let img = open(IMG_PATH).expect("Failure to load/open image");
    let (width, height) = img.dimensions();
    println!("width: {width}, height: {height}");

    let saving_path: String = String::from("assets/sample_img");

    let path = saving_path.clone() + ".png";
    save_image(&img, path.as_str(), ImageFormat::Png);
    let path = saving_path.clone() + ".webp";
    save_image(&img, path.as_str(), ImageFormat::WebP);

    let resized_image = resize_image(IMG_PATH, 512, 512);
    let saving_path: String = String::from("assets/sample_img_RESIZED");

    let path = saving_path.clone() + ".png";
    save_image(&resized_image, path.as_str(), ImageFormat::Png);
    let path = saving_path.clone() + ".webp";
    save_image(&resized_image, path.as_str(), ImageFormat::WebP);

    let saving_path: String = String::from("assets/sample_img_ROTATED_");

    let angle: u32 = 90;
    let path = saving_path.clone() + angle.to_string().as_str() + ".png";
    let rotated_image = rotate_image(IMG_PATH, angle);
    save_image(&rotated_image, path.as_str(), ImageFormat::Png);

    let angle: u32 = 180;
    let path = saving_path.clone() + angle.to_string().as_str() + ".png";
    let rotated_image = rotate_image(IMG_PATH, angle);
    save_image(&rotated_image, path.as_str(), ImageFormat::Png);

    let angle: u32 = 270;
    let path = saving_path.clone() + angle.to_string().as_str() + ".png";
    let rotated_image = rotate_image(IMG_PATH, angle);
    save_image(&rotated_image, path.as_str(), ImageFormat::Png);

    let angle: u32 = 67;
    let path = saving_path.clone() + angle.to_string().as_str() + ".png";
    let rotated_image = rotate_image(IMG_PATH, angle);
    save_image(&rotated_image, path.as_str(), ImageFormat::Png);
}
