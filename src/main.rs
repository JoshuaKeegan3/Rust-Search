mod directory;
mod lexer;
mod server;
mod tokeniser;
mod util;
use directory::*;
use server::*;
use std::collections::HashMap;
use std::fs::File;
use std::io;

fn main1() -> io::Result<()> {
    let mut document_data = HashMap::<String, HashMap<String, usize>>::new();
    let dir = "doc/html";

    process_file_tree(&mut document_data, dir);

    let out_path = "tf.json";
    let out_file = File::create(out_path)?;
    serde_json::to_writer(out_file, &document_data).expect("no error");

    Ok(())
}

fn main() {
    start_server();
}
