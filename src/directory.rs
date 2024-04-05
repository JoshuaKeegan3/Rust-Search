use super::lexer::*;
use super::tokeniser::*;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

// TODO: Multi-Threading

pub fn process_file_tree(
    document_data: &mut HashMap<String, HashMap<String, usize>>,
    dir: &str,
) -> () {
    let parent_path = fs::read_dir(dir).unwrap_or_else(|err| {
        eprintln!("no such dir {dir}");
        eprintln!("{err}");
        exit(1);
    });
    for file in parent_path {
        let file_path = file
            .unwrap_or_else(|err| {
                eprintln!("error processing file path");
                eprintln!("{err}");
                exit(1);
            })
            .path();
        if file_path.ends_with(".nojekyll") || file_path.ends_with(".DStore") {
            continue;
        }
        let file_extention = file_path.extension().unwrap_or_else(|| OsStr::new(""));
        if file_extention == "html" {
            process_html(file_path, document_data)
        } else if file_extention == "" {
            process_file_tree(document_data, file_path.to_str().unwrap());
        }
    }
    ()
}

fn process_html(file_path: PathBuf, document_data: &mut HashMap<String, HashMap<String, usize>>) {
    let content = tokenise_html(&file_path)
        .unwrap_or_else(|err| {
            eprintln!("error tokenising_html");
            eprintln!("{err}");
            "".to_string()
        })
        .chars()
        .collect::<Vec<_>>();
    let mut term_frequency_map = HashMap::<String, usize>::new();
    for token in Lexer::new(&content) {
        let term = token
            .iter()
            .map(|x| x.to_ascii_lowercase())
            .collect::<String>();

        if let Some(count) = term_frequency_map.get_mut(&term) {
            *count += 1;
        } else {
            term_frequency_map.insert(term, 1);
        }
    }
    document_data.insert(
        file_path.as_path().display().to_string(),
        term_frequency_map,
    );
}
