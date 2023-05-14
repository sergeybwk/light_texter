use std::vec;
use std::{fs, path::PathBuf};

use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};

pub fn create_pdf(file_path: Option<PathBuf>) {
    // TODO implement open created pdf-file

    let str_lines = parsing_str(file_path);

    let mut doc = Document::with_version("1.5");

    let font_id = doc.add_object(dictionary! {
        // type of dictionary
        "Type" => "Font",
        // type of font, type1 is simple postscript font
        "Subtype" => "Type1",
        // basefont is postscript name of font for type1 font.
        // See PDF reference document for more details
        "BaseFont" => "Courier",
    });

    // font dictionaries need to be added into resource dictionaries
    // in order to be used.
    // Resource dictionaries can contain more than just fonts,
    // but normally just contains fonts
    // Only one resource dictionary is allowed per page tree root
    let resources_id = doc.add_object(dictionary! {
        // fonts are actually triplely nested dictionaries. Fun!
        "Font" => dictionary! {
            // F1 is the font name used when writing text.
            // It must be unique in the document. It does not
            // have to be F1
            "F1" => font_id,
        },
    });

    let content = Content {
        operations: vec![
            // BT begins a text element. it takes no operands
            Operation::new("BT", vec![]),
            // Tf specifies the font and font size. Font scaling is complicated in PDFs. Reference
            // the reference for more info.
            // The info() methods are defined based on their paired .from() methods (this
            // functionality is built into rust), and are converting the provided values into
            // An enum that represents the basic object types in PDF documents.
            Operation::new("Tf", vec!["F1".into(), 48.into()]),
            // Td adjusts the translation components of the text matrix. When used for the first
            // time after BT, it sets the initial text position on the page.
            // Note: PDF documents have Y=0 at the bottom. Thus 600 to print text near the top.
            Operation::new("Td", vec![100.into(), 600.into()]),
            // Tj prints a string literal to the page. By default, this is black text that is
            // filled in. There are other operators that can produce various textual effects and
            // colors
            Operation::new("Tj", vec![Object::string_literal("12313124")]),
            // ET ends the text element
            Operation::new("ET", vec![]),
        ],
    };

    let pages_id = doc.new_object_id();

    // content is a stream of encoded content data.
    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));

    // Page is a dictionary that represents one page of a PDF file.
    // It has a type, parent and contents
    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    });

    // Again, pages is the root of the page tree. The ID was already created
    // at the top of the page, since we needed it to assign to the parent element of the page
    // dictionary
    //
    // This is just the basic requirements for a page tree root object. There are also many
    // additional entries that can be added to the dictionary if needed. Some of these can also be
    // defined on the page dictionary itself, and not inherited from the page tree root.
    let pages = dictionary! {
        // Type of dictionary
        "Type" => "Pages",
        // Vector of page IDs in document. Normally would contain more than one ID and be produced
        // using a loop of some kind
        "Kids" => vec![page_id.into()],
        // Page count
        "Count" => 1,
        // ID of resources dictionary, defined earlier
        "Resources" => resources_id,
        // a rectangle that defines the boundaries of the physical or digital media. This is the
        // "Page Size"
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    };

    // using insert() here, instead of add_object() since the id is already known.
    doc.objects.insert(pages_id, Object::Dictionary(pages));

    // Creating document catalog.
    // There are many more entries allowed in the catalog dictionary.
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    // Root key in trailer is set here to ID of document catalog,
    // remainder of trailer is set during doc.save().
    doc.trailer.set("Root", catalog_id);
    doc.compress();

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

    println!("{:?}", cur_path);
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
