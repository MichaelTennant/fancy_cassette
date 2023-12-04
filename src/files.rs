use std::path::Path;
use regex::Regex;
use std::fs;

pub fn list(music_files: &mut Vec<String>) {
    // Get the OS music directory
    let music_directory = dirs::audio_dir().unwrap();
    let music_path: &Path = music_directory.as_path();

    // Get the paths top the files in that music dir
    let paths = fs::read_dir(music_path).expect("Failed to read OS music folder");

    // Loop through those paths and add them to a new list of strings
    for path in paths {
        music_files.push(path.unwrap().path().display().to_string());
        // println!("Name: {}", path.unwrap().path().display());
    }

    // Yeet all but .mp3 files
    music_files.retain(|test_string| { is_mp3(&test_string) })
}

fn is_mp3(in_str: &str) -> bool {
    // Regex to match for .mp3 at the end of the filename
    let mp3_regex: Regex = Regex::new("\\.mp3$").unwrap();

    // Return true if we find it and false if we don't
    return match mp3_regex.find(in_str) {
        Some(_) => true,
        None => false,
    };
}
