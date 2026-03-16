use std::fs;
use std::path::Path;

fn main() {

    let folder = "./files";

    let entries = fs::read_dir(folder).expect("Failed to read folder");

    for entry in entries {

        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {

            let extension = path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");

            let category = match extension {
                "png" | "jpg" | "jpeg" => "Images",
                "mp3" | "wav" => "Music",
                "pdf" | "txt" | "docx" => "Documents",
                _ => "Others"
            };

            let target_folder = format!("{}/{}", folder, category);

            fs::create_dir_all(&target_folder).unwrap();

            let filename = path.file_name().unwrap();
            let new_path = Path::new(&target_folder).join(filename);

            fs::rename(&path, new_path).unwrap();
        }
    }

    println!("Files organized successfully!");
}