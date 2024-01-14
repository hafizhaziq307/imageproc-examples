// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api; 

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            api::bilateral_filter,
            api::median_filter,
            api::gaussian_blur,
            api::sharpen3x3,
            api::sharpen_gaussian,
            api::stretch_contrast,
            api::threshold,
            api::dilate,
            api::distance_transform,
            api::canny,
            api::gaussian_noise,
            api::salt_and_pepper_noise,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
