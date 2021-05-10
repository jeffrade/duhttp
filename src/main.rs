use std::env;
use std::io::Cursor;
use tiny_http::{Request, Response, Server};

fn main() {
    let listen_addr = "0.0.0.0:".to_owned() + &listening_port();
    let server = Server::http(&listen_addr).unwrap();

    for request in server.incoming_requests() {
        let response = handle(&request);
        request
            .respond(response)
            .expect("Doh! Could not respond to request ¯\\_(ツ)_/¯");
    }
}

fn handle(request: &Request) -> Response<Cursor<Vec<u8>>> {
    println!(
        "method: {:?},\nurl: {:?},\nheaders: {:?}",
        request.method(),
        request.url(),
        request.headers()
    );

    Response::from_string("ok")
}

fn listening_port() -> String {
    match env::var("DUHTTP_PORT") {
        Ok(port) => port,
        Err(_) => String::from("8080"),
    }
}
