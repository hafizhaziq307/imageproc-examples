use imageproc::{filter, noise, edges, distance_transform, morphology, contrast};
use serde_json::{json, Value};
use base64::{engine::general_purpose, Engine as _};
use image::{open, GrayImage, RgbImage, ImageFormat};
use std::io::Cursor;


#[tauri::command(async)]
pub fn bilateral_filter(path: String) -> Value {
    let image: GrayImage = open(&path).expect("File not found!").into_luma8();

    let image_format: ImageFormat = ImageFormat::from_path(&path).unwrap();

    let filtered = filter::bilateral_filter(&image, 10, 10., 3.);

    let base64img: String = gray_image_to_base64(&filtered, image_format);

    return json!({
        "image": &base64img,
    });
}

#[tauri::command(async)]
pub fn median_filter(path: String) -> Value {
    let image = open(&path).expect("File not found!").into_rgb8();

    let image_format: ImageFormat = ImageFormat::from_path(&path).unwrap();

    let filtered = filter::median_filter(&image, 1, 1);

    let base64img: String = rgb_image_to_base64(&filtered, image_format);

    return json!({
        "image": &base64img,
    });
}

#[tauri::command(async)]
pub fn gaussian_blur(path: String) -> Value {
    let image = open(&path).expect("File not found!").into_rgb8();

    let image_format: ImageFormat = ImageFormat::from_path(&path).unwrap();

    let filtered = filter::gaussian_blur_f32(&image, 1.0);

    let base64img: String = rgb_image_to_base64(&filtered, image_format);

    return json!({
        "image": &base64img,
    });
}

#[tauri::command(async)]
pub fn sharpen3x3(path: String) -> Value {
    let image: GrayImage = open(&path).expect("File not found!").into_luma8();

    let image_format: ImageFormat = ImageFormat::from_path(&path).unwrap();

    let filtered = filter::sharpen3x3(&image);

    let base64img: String = gray_image_to_base64(&filtered, image_format);

    return json!({
        "image": &base64img,
    });
}

#[tauri::command(async)]
pub fn sharpen_gaussian(path: String) -> Value {
    let image: GrayImage = open(&path).expect("File not found!").into_luma8();

    let image_format: ImageFormat = ImageFormat::from_path(&path).unwrap();

    let filtered = filter::sharpen_gaussian(&image, 1.5, 1.0);

    let base64img: String = gray_image_to_base64(&filtered, image_format);

    return json!({
        "image": &base64img,
    });
}

#[tauri::command(async)]
pub fn stretch_contrast(path: String) -> Value {
    let image: GrayImage = open(&path).expect("File not found!").into_luma8();

    let image_format: ImageFormat = ImageFormat::from_path(&path).unwrap();

    let filtered = contrast::stretch_contrast(&image, 20, 200);

    let base64img: String = gray_image_to_base64(&filtered, image_format);

    return json!({
        "image": &base64img,
    });
}

#[tauri::command(async)]
pub fn threshold(path: String) -> Value {
    let image: GrayImage = open(&path).expect("File not found!").into_luma8();

    let image_format: ImageFormat = ImageFormat::from_path(&path).unwrap();

    let filtered = contrast::threshold(&image, 50);

    let base64img: String = gray_image_to_base64(&filtered, image_format);

    return json!({
        "image": &base64img,
    });
}

#[tauri::command(async)]
pub fn dilate(path: String) -> Value {
    let image: GrayImage = open(&path).expect("File not found!").into_luma8();

    let image_format: ImageFormat = ImageFormat::from_path(&path).unwrap();

    let filtered = morphology::dilate(&image, distance_transform::Norm::L1, 10);

    let base64img: String = gray_image_to_base64(&filtered, image_format);

    return json!({
        "image": &base64img,
    });
}

#[tauri::command(async)]
pub fn distance_transform(path: String) -> Value {
    let image: GrayImage = open(&path).expect("File not found!").into_luma8();

    let image_format: ImageFormat = ImageFormat::from_path(&path).unwrap();

    let filtered = distance_transform::distance_transform(&image, distance_transform::Norm::LInf);

    let base64img: String = gray_image_to_base64(&filtered, image_format);

    return json!({
        "image": &base64img,
    });
}

#[tauri::command(async)]
pub fn canny(path: String) -> Value {
    let image: GrayImage = open(&path).expect("File not found!").into_luma8();

    let image_format: ImageFormat = ImageFormat::from_path(&path).unwrap();

    let filtered = edges::canny(&image, 50.0, 100.0);

    let base64img: String = gray_image_to_base64(&filtered, image_format);

    return json!({
        "image": &base64img,
    });
}

#[tauri::command(async)]
pub fn gaussian_noise(path: String) -> Value {
    let image: RgbImage = open(&path).expect("File not found!").into_rgb8();

    let image_format: ImageFormat = ImageFormat::from_path(&path).unwrap();

    let filtered: RgbImage = noise::gaussian_noise(&image, 0.0, 10.0, 25);

    let base64img: String = rgb_image_to_base64(&filtered, image_format);

    return json!({
        "image": &base64img,
    });
}

#[tauri::command(async)]
pub fn salt_and_pepper_noise(path: String) -> Value {
    let image: RgbImage = open(&path).expect("File not found!").into_rgb8();

    let image_format: ImageFormat = ImageFormat::from_path(&path).unwrap();

    let filtered: RgbImage = noise::salt_and_pepper_noise(&image, 0.01, 42);

    let base64img: String = rgb_image_to_base64(&filtered, image_format);

    return json!({
        "image": &base64img,
    });
}




fn gray_image_to_base64(img: &GrayImage, extension: ImageFormat) -> String {
    let mut image_data: Vec<u8> = Vec::new();

    img.write_to(&mut Cursor::new(&mut image_data), extension).unwrap();

    return general_purpose::STANDARD.encode(image_data);
}

fn rgb_image_to_base64(img: &RgbImage, extension: ImageFormat) -> String {
    let mut image_data: Vec<u8> = Vec::new();

    img.write_to(&mut Cursor::new(&mut image_data), extension).unwrap();

    return general_purpose::STANDARD.encode(image_data);
}
