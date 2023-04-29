use lopdf::Document;

pub fn create_pdf() {
    let mut doc = Document::with_version("1.5");

    let pages_id = doc.new_object_id();

    doc.save("example.pdf");
}
