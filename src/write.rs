use std::{fs::File, fs::OpenOptions, io::Write};

// Writing the line to the file
pub fn write_to_file(str: &str) {
    // let mut file = File::create("text.txt").expect("failed to create the file");
    let mut filetest = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("text.txt")
        .expect("Error dealing with file");

    writeln!(filetest, "{}", str).expect("something is wrong with writing in file");
}
