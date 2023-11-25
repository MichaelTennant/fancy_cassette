use std::path::Path;
use std::fs;

pub fn list() {
    let music_directory = dirs::audio_dir().unwrap();
    let music_path: &Path = music_directory.as_path();

    let paths = fs::read_dir(music_path).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }
}
