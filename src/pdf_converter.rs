use lopdf::Document;

pub fn create_pdf() {
    // TODO implement open created pdf-file

    let mut doc = Document::with_version("1.5");

    let pages_id = doc.new_object_id();

    doc.save("example.pdf");
}
