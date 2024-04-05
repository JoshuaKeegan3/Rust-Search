use super::lexer::Lexer;
use crate::util::tfidf;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let mut lines = buf_reader.lines();
    let request_line = lines.next().unwrap().unwrap();

    println!("{request_line:?}");

    match request_line.as_str() {
        "GET / HTTP/1.1" => {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("src/front-end/index.html").unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
        "GET /index.js HTTP/1.1" => {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("src/front-end/index.js").unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
        "POST /tf/search HTTP/1.1" => {
            let _host = lines.next().unwrap().unwrap();
            let _accept = lines.next().unwrap().unwrap();
            let _sec_fetch_site = lines.next().unwrap().unwrap();
            let _lang = lines.next().unwrap().unwrap();
            let _accept_encoding = lines.next().unwrap().unwrap();
            let _sec_fetch_mode = lines.next().unwrap().unwrap();
            let _content_type = lines.next().unwrap().unwrap();
            let _origin = lines.next().unwrap().unwrap();
            let _user_agent = lines.next().unwrap().unwrap();
            let _referer = lines.next().unwrap().unwrap();
            let _content_length = lines.next().unwrap().unwrap();
            let _connection = lines.next().unwrap().unwrap();
            let _sec_fetch_dist = lines.next().unwrap().unwrap();
            let _blank = lines.next().unwrap().unwrap();
            let body = lines.next().unwrap().unwrap();

            // use the body to search for the terms.
            let score = search_query(&body.chars().collect::<Vec<_>>());

            let status_line = "HTTP/1.1 200 OK";
            let mut test_ans: Vec<_> = Vec::<_>::new();
            test_ans.push((&score[0].0, score[0].1));
            test_ans.push((&score[1].0, score[1].1));

            let contents = serde_json::to_string(&test_ans).unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
            stream.write_all(response.as_bytes()).unwrap();
        }
        _ => {
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let contents = fs::read_to_string("src/front-end/404.html").unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap(); // some other request
        }
    }
}

pub fn search_query(query: &[char]) -> Vec<(String, f32)> {
    let mut result = Vec::new();
    let tokens = Lexer::new(&query).collect::<Vec<_>>();

    let file = File::open("tf.json").unwrap();
    let dir_map: HashMap<String, HashMap<String, usize>> =
        serde_json::from_reader(file).expect("file should be proper JSON");

    for (path, doc) in &dir_map {
        let mut rank = 0f32;
        for token in &tokens {
            rank += tfidf(
                token.iter().collect::<String>().as_str(),
                doc.clone(),
                &dir_map,
            );
        }
        // TODO: investigate the sources of NaN
        if !rank.is_nan() {
            result.push((path.clone(), rank));
        }
    }
    result.sort_by(|(_, rank1), (_, rank2)| {
        rank1
            .partial_cmp(rank2)
            .expect(&format!("{rank1} and {rank2} are not comparable"))
    });
    result.reverse();
    result
}
