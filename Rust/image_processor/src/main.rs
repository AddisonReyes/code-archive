use image::{DynamicImage, GenericImageView, ImageFormat, open};

fn main() {
    let saving_path: String = String::from("assets/sample_img");
    const IMG_PATH: &str = "assets/sample_img.jpg";
    let img = open(IMG_PATH).expect("Failure to load/open image");

    let (width, height) = img.dimensions();
    println!("width: {width}, height: {height}");

    let path = saving_path.clone() + ".png";
    img.save_with_format(path, ImageFormat::Png)
        .expect("Failed to save image as PNG");

    let path = saving_path.clone() + ".webp";
    img.save_with_format(path, ImageFormat::WebP)
        .expect("Failed to save image as WEBP");
}
