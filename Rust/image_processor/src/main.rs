use image::{DynamicImage, GenericImageView, ImageFormat, open};

mod utils;

const IMG_PATH: &str = "assets/sample_img.jpg";

#[allow(dead_code)]
fn saving_images(img: &DynamicImage) {
    println!("- - - Saving images - - -");

    let saving_path: String = String::from("assets/sample_img");

    let path = saving_path.clone() + ".png";
    utils::save_image(img, path.as_str(), ImageFormat::Png);
    let path = saving_path.clone() + ".webp";
    utils::save_image(img, path.as_str(), ImageFormat::WebP);
}

#[allow(dead_code)]
fn resized_images() {
    println!("- - - Resized images - - -");

    let resized_image = utils::resize_image(IMG_PATH, 512, 512);
    let saving_path: String = String::from("assets/sample_img_RESIZED");

    let path = saving_path.clone() + ".png";
    utils::save_image(&resized_image, path.as_str(), ImageFormat::Png);
    let path = saving_path.clone() + ".webp";
    utils::save_image(&resized_image, path.as_str(), ImageFormat::WebP);
}

#[allow(dead_code)]
fn rotated_images() {
    println!("- - - Rotated images - - -");

    let saving_path: String = String::from("assets/sample_img_ROTATED_");

    let angle: u32 = 90;
    let path = saving_path.clone() + angle.to_string().as_str() + ".png";
    let rotated_image = utils::rotate_image(IMG_PATH, angle);
    utils::save_image(&rotated_image, path.as_str(), ImageFormat::Png);

    let angle: u32 = 180;
    let path = saving_path.clone() + angle.to_string().as_str() + ".png";
    let rotated_image = utils::rotate_image(IMG_PATH, angle);
    utils::save_image(&rotated_image, path.as_str(), ImageFormat::Png);

    let angle: u32 = 270;
    let path = saving_path.clone() + angle.to_string().as_str() + ".png";
    let rotated_image = utils::rotate_image(IMG_PATH, angle);
    utils::save_image(&rotated_image, path.as_str(), ImageFormat::Png);

    let angle: u32 = 67;
    let path = saving_path.clone() + angle.to_string().as_str() + ".png";
    let rotated_image = utils::rotate_image(IMG_PATH, angle);
    utils::save_image(&rotated_image, path.as_str(), ImageFormat::Png);
}

#[allow(dead_code)]
fn resized_images_with_ratio() {
    println!("- - - Resized images with maintaing ratio - - -");

    let (w, h) = (Some(200), None);
    let resized_image = utils::resize_image_maintain_ratio(IMG_PATH, w, h);
    let saving_path: String = String::from("assets/sample_img_RESIZED_RATIO");
    let path = saving_path.clone() + ".jpg";

    utils::save_image(&resized_image, path.as_str(), ImageFormat::Jpeg);
}

#[allow(dead_code)]
fn cropped_image(img: &DynamicImage) {
    println!("- - - Cropped image - - -");

    let cropped_img = utils::crop_image(img, 3800, 1600, 960, 540);
    let saving_path: String = String::from("assets/sample_img_CROPPED.png");

    utils::save_image(&cropped_img, saving_path.as_str(), ImageFormat::Png);
}

fn main() {
    println!("- - - Image processing - - -");

    let img = open(IMG_PATH).expect("Failure to load/open image");
    let (width, height) = img.dimensions();

    println!("IMG_PATH: {IMG_PATH}");
    println!("width: {width}, height: {height}");

    // saving_images(&img);
    // resized_images();
    // rotated_images();
    // resized_images_with_ratio();
    cropped_image(&img);
}
