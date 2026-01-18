use std::{
    env, fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::Path,
};

use wgpu_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Serving on http://127.0.0.1:7878");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => format!("Error: {e}"),
        None => "Client Disconected".into(),
    };

    let path = request_line.split_whitespace().nth(1).unwrap_or("/");

    let filename = if path == "/" {
        "static/index.html"
    } else {
        &(format!("static{}", path))
    };

    if Path::new(&filename).is_file() && is_path_safe(filename) {
        let contents = fs::read(filename).unwrap();
        let length = contents.len();
        let content_type = get_content_type(filename);

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n"
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    } else {
        let response = "HTTP/1.1 404 NOT FOUND{}\r\n\r\n static/404.html";
        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn get_content_type(filename: &str) -> &'static str {
    if filename.ends_with(".html") {
        "text/html"
    } else if filename.ends_with(".wasm") {
        "application/wasm"
    } else if filename.ends_with(".js") {
        "application/javascript"
    } else if filename.ends_with(".css") {
        "text/css"
    } else if filename.ends_with(".png") {
        "image/png"
    } else if filename.ends_with(".jpg") {
        "image/jpeg"
    } else if filename.ends_with(".obj") {
        "mesh/obj"
    } else {
        "text/plain"
    }
}
fn is_path_safe(user_input_path: &str) -> bool {
    let base_dir = env::current_dir().expect("Invalid base directory");
    let canonical_path = match fs::canonicalize(base_dir.join(user_input_path)) {
        Ok(s) => s,
        Err(_) => return false,
    };

    canonical_path.starts_with(base_dir)
}
