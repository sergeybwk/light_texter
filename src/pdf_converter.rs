use std::{fs, path::PathBuf};

use lopdf::Document;

pub fn create_pdf(file_path: Option<PathBuf>) {
    // TODO implement open created pdf-file

    let str = parsing_str(file_path);

    let mut doc = Document::with_version("1.5");

    let pages_id = doc.new_object_id();

    doc.save("example.pdf");
}

pub fn parsing_str(file_path: Option<PathBuf>) -> Vec<(String, bool)> {
    let cur_path: PathBuf = match file_path {
        Some(p) => p,
        None => {
            let mut entries: Vec<fs::DirEntry> = fs::read_dir(".")
                .expect("Couldn't access local directory")
                .flatten() // Remove failed
                .collect();
            entries.sort_by_cached_key(|f| f.metadata().unwrap().modified().unwrap());
            entries[0].path()
        }
    };
    let str = fs::read_to_string(cur_path).unwrap();
    let lines: Vec<String> = str
        .lines()
        .map(|s| s.to_owned())
        .filter(|line| !line.is_empty())
        .collect();
    let mut res: Vec<(String, bool)> = Vec::new();
    for i in lines {
        let temp = i.to_owned().into_bytes();
        match (temp[0], temp[1]) {
            (b'h', b'1') => res.push((i[2..].to_string(), true)),
            _ => res.push((i, false)),
        }
    }
    println!("{:?}", res);

    res
}
