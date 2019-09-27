//! Kauri daemon
//!
//! The following module contains the Kauri daemon. A simple HTTP server that
//! runs silently in the background, providing performance critical services to
//! the Electron front end. This portion of the project most notable includes
//! the following modules:
//!
//! - A basic `document` implementation.
//! - Parsers for common document formats (`parsers`).
//! - Exporters for common file formats.

#![warn(missing_docs)]

extern crate tiny_http;

mod controllers;
mod document;
mod parsers;

use tiny_http::Response;

/// Main entry point
/// Establishes the simple HTTP server, and listens for requests.
fn main() {
    let addr = "127.0.0.1:3000";
    let server = tiny_http::Server::http(addr).unwrap();
    println!("Listening on http://{}", addr);

    loop {
        let request = match server.recv() {
            //server.recv() blocks until a request actually comes
            Ok(rq) => rq,
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        };
        handle_request(request);
    }
}

/// Takes a request and responds accordingly
fn handle_request(mut request: tiny_http::Request) {
    let mut resp_normal: Option<Response<std::io::Cursor<Vec<u8>>>> = None;
    let mut resp_error: Option<Response<std::io::Empty>> = None;
    match request.url() {
        "/load" => resp_normal = Some(controllers::load::load_controller(&mut request)),
        _ => resp_error = Some(Response::empty(404)),
    };

    if let Some(mut resp) = resp_normal {
        // Disable CORS for normal responses
        resp.add_header(
            tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap(),
        );
        send_response(request, resp);
    } else {
        // No need to disable CORS here, since the response has no data associated with it
        send_response(request, resp_error.unwrap());
    }
}

/// Attempts to send the response for the given request, will print an error message if it fails
fn send_response<T: std::io::Read>(request: tiny_http::Request, response: tiny_http::Response<T>) {
    if let Err(e) = request.respond(response) {
        println!("error: {}", e);
        return;
    }
}
