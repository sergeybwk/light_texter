use std::{
    fs::File,
    fs::OpenOptions,
    fs::{self, create_dir},
    io::Write,
};

// TODO choosing file
// Writing the line to the file
pub fn write_to_file(str: &str) {
    create_dir("files");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("files/text.txt")
        .expect("Error dealing with file");

    writeln!(file, "{}", str).expect("something is wrong with writing in file");
}

pub fn read_from_file() -> Result<String, std::io::Error> {
    fs::read_to_string("files/text.txt")
}
