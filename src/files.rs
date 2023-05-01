use std::{
    fs::{self, create_dir, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

// TODO choosing file
// Writing the line to the file
pub fn write_to_file(str: &str, path: Option<PathBuf>) {
    let cur_path = match path {
        Some(v) => v,
        None => {
            create_dir("files");
            let mut path = PathBuf::from("files");
            let mut index = 0;
            path.push(format!("unnamed_{}.txt", index));
            while PathBuf::from(&path).exists() {
                index += 1;
                path = PathBuf::from(format!("files/unnamed_{}.txt", index));
            }
            path
        }
    };
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(cur_path)
        .expect("Error dealing with file");

    writeln!(file, "{}", str).expect("something is wrong with writing in file");
}

pub fn read_from_file(path: Option<PathBuf>) -> String {
    let res: String;
    if path != None {
        res = match fs::read_to_string(path.expect("err path")) {
            Ok(v) => v,
            _ => "".to_owned(),
        };
    } else {
        res = "".to_string();
    }

    res
}
