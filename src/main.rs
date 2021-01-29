use std::fs;
use std::vec::Vec;
use std::path::Path;
use std::env;
use regex::Regex;
use id3::Tag;

const ROOT_DIRECTORY :&str = "./sorted";

fn get_mp3_info(path: &str) -> (String, String){
    let tag= Tag::read_from_path(path).expect("Failed to read mp3 file");
    let album = match tag.album() {
        Some(n) => n,
        None => "no album",
    };
    let artist = match tag.artist() {
        Some(n) => n,
        None => "no artist",
    };
    (artist.to_string(), album.to_string())
}

fn main() {
    let re = Regex::new(r"/.+/").unwrap();
    let mp3_regex = Regex::new(r".+\.mp3").unwrap();
    let current_path = env::current_exe().unwrap();
    let current_path = current_path.to_str().unwrap();
    let caps = re.captures(current_path).unwrap();
    let current_path = &caps[0];
    let paths = fs::read_dir(current_path.to_string() + "/").expect("File system Error.");
    let mut mp3_file_list : Vec<String> = Vec::new();

    for path in paths {
        let path_string = path.unwrap().file_name().into_string().unwrap();
        if mp3_regex.is_match(&path_string) {
            mp3_file_list.push(path_string);
        }
    }
    for path in mp3_file_list.iter(){
        let music_path = current_path.to_string() + &path;
        let (_artist, album) = get_mp3_info(&music_path);
        let output_path = current_path.to_string() + ROOT_DIRECTORY + "/" + &album;
        if !Path::new(&output_path).exists() {
            fs::create_dir_all(&output_path).expect("Failed to create directory.");
        }
        let move_path = output_path.clone() + "/" + &path;
        fs::rename(&music_path, &move_path).expect("Failed to rename.");
    }
    println!("Done!");
}
