// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Local;
use image::io::Reader as ImageReader;
use image::{DynamicImage, EncodableLayout}; // Using image crate: https://github.com/image-rs/image
use webp::{Encoder, WebPMemory}; // Using webp crate: https://github.com/jaredforth/webp

use directories::UserDirs;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[tauri::command(async)]
fn image_to_webp(files: Vec<String>) -> String {
    let mut directory: Option<PathBuf> = None;

    if let Some(downloads_directory) = get_downloads_directory() {
        if let Some(dir) = generate_directory(downloads_directory) {
            for file in files.iter() {
                let path = Path::new(file);
                process_image(path, &dir)
            }
            directory = Some(dir);
        }
    }

    directory.map_or("".to_string(), |dir| dir.to_string_lossy().to_string())
}

fn get_downloads_directory() -> Option<PathBuf> {
    if let Some(user_dirs) = UserDirs::new() {
        return UserDirs::download_dir(&user_dirs).map(PathBuf::from);
    }
    None
}

fn generate_directory(parent_directory: PathBuf) -> Option<PathBuf> {
    let formatted_date = Local::now().format("%d-%m-%Y_%H-%M-%S");
    println!("{}", formatted_date);

    let directory: PathBuf = Path::join(
        &parent_directory,
        format!(
            "{}/TOP_HAT_Images_{}",
            parent_directory.as_os_str().to_str().unwrap(),
            formatted_date
        ),
    );

    if let Err(err) = std::fs::create_dir(&directory) {
        eprintln!("Error creating directory: {}", err);
        return None;
    }

    Some(directory)
}

fn process_image(file: &Path, directory: &Path) {
    if let Some(file_name) = file.file_stem() {
        let path: PathBuf = Path::join(directory, format!("{}.webp", file_name.to_str().unwrap()));

        if let Ok(mut webp_image) = File::create(path) {
            let image: DynamicImage = ImageReader::open(&file).unwrap().decode().unwrap();

            // Make webp::Encoder from DynamicImage
            let encoder: Encoder = Encoder::from_image(&image).unwrap();

            // Encode image into WebPMemory
            let encoded_webp: WebPMemory = encoder.encode(80f32);

            webp_image.write_all(encoded_webp.as_bytes());
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![image_to_webp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
