use std::{fs::File, fs::OpenOptions, io::Write};

// Writing the line to the file
pub fn write_to_file(str: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("text.txt")
        .expect("Error dealing with file");

    writeln!(file, "{}", str).expect("something is wrong with writing in file");
}
