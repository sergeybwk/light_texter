use std::vec;
use std::{fs, path::PathBuf};

use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};

pub fn create_pdf(file_path: Option<PathBuf>) {
    let str_lines = parsing_str(file_path);

    let mut doc = Document::with_version("1.5");

    doc.save("example.pdf");
}

pub fn parsing_str(file_path: Option<PathBuf>) -> Vec<(String, bool)> {
    let cur_path: PathBuf = match file_path {
        Some(p) => p,
        None => {
            let mut entries: Vec<fs::DirEntry> = fs::read_dir("./files")
                .expect("Couldn't access local directory")
                .flatten() // Remove failed
                .collect();
            entries.sort_by_cached_key(|f| f.metadata().unwrap().modified().unwrap());
            entries[0].path()
        }
    };

    // println!("{:?}", cur_path);
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
    // println!("{:?}", res);

    res
}
