use std::fs::File;
use std::io;
use std::path::Path;

pub fn tokenise_html<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = File::open(file_path).expect("no such file: {file_path?}");
    let mut content = html2text::from_read(&file, 20);
    content = content.replace("\n", " ");
    Ok(content)
}
