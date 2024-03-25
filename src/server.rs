use tiny_http;

pub fn handle_requests(port: u32) -> anyhow::Result<()> {
    let local_url = format!("127.0.0.1:{port}");
    let server = tiny_http::Server::http(local_url).expect("Failed to bind to port!");
    for request in server.incoming_requests() {
        println!("URL: {}", request.url());
        let res = tiny_http::Response::from_string("What's up my man?");
        request.respond(res).expect("I don't care!");
    }
    Ok(())
}
