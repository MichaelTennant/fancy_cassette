mod files;

fn main() {
    println!("Hello, world!");

    let mut music_files: Vec<String> = Vec::new();
    files::list(&mut music_files);

    println!("{:?}", music_files)
}
