use id3::Tag;
use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::vec::Vec;

const ROOT_DIRECTORY: &str = "sorted";

fn get_mp3_info(path: &Path) -> Result<(String, String), &str> {
    if let Ok(tag) = Tag::read_from_path(path) {
        let album = match tag.album() {
            Some(n) => n,
            None => "no album",
        };
        let artist = match tag.artist() {
            Some(n) => n,
            None => "no artist",
        };
        Ok((artist.to_string(), album.to_string()))
    } else {
        if let Some(s) = path.to_str() {
            Err(s)
        } else {
            Err("Unexpected error")
        }
    }
}

fn main() {
    let mp3_regex = Regex::new(r".+\.mp3?").unwrap();
    let current_path = env::current_exe().unwrap();
    let current_path = current_path.parent().unwrap();
    let paths = fs::read_dir(current_path).unwrap();
    let mut mp3_file_list: Vec<String> = Vec::new();

    for path in paths {
        let path_string = path.unwrap().path();
        let path_string = path_string.file_name().unwrap();
        let path_string = path_string.to_str().unwrap();
        if mp3_regex.is_match(path_string) {
            mp3_file_list.push(path_string.to_string());
        }
    }

    for path in mp3_file_list.iter() {
        let mut music_path = PathBuf::from(current_path);
        music_path.push(path);
        match get_mp3_info(&music_path) {
            Ok((_artist, album)) => {
                let mut output_path = PathBuf::from(current_path);
                output_path.push(ROOT_DIRECTORY);
                output_path.push(album);
                if !output_path.exists() {
                    fs::create_dir_all(&output_path).expect("Failed to create directory.");
                }
                output_path.push(path);
                fs::rename(music_path, output_path).expect("Failed to rename.");
            }
            Err(e) => {
                println!("Failed to read mp3 file : {}", e);
            }
        }
    }
    println!("Done!");
}
