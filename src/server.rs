use crate::index::DirIndex;
use tiny_http;
use url::Url;

pub fn handle_requests(port: u32, dir_idx: &DirIndex) -> anyhow::Result<()> {
    let local_url = format!("127.0.0.1:{port}");
    let server = tiny_http::Server::http(local_url).expect("Failed to bind to port!");
    for request in server.incoming_requests() {
        let url = format!("http://localhost:{port}{}", request.url());
        println!("{url}");
        let url = Url::parse(&url)?;
        let params = Url::query_pairs(&url);
        'inner: for i in params {
            if i.0.eq("query") {
                let search_results = dir_idx.search_term(&i.1);
                let serialized_output = serde_json::to_string(&search_results)?;
                let res = tiny_http::Response::from_data(serialized_output);

                let cors_header =
                    tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..])
                        .expect("Fuck this!");
                let cors_another_header =
                    tiny_http::Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"*"[..])
                        .expect("Another fuck you!");
                request
                    .respond(
                        res.with_header(cors_another_header)
                            .with_header(cors_header),
                    )
                    .expect("I don't care!");
                break 'inner;
            }
        }
    }
    Ok(())
}
