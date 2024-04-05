use core::f64;
use std::collections::HashMap;

use crate::index::DirIndex;
use serde::Serialize;
use std::path::Path;
use tiny_http;
use url::Url;

#[derive(Serialize, Debug)]
struct QueryResponse {
    query: String,
    results: HashMap<String, f64>,
}

fn canonicalize_path<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    return path
        .canonicalize()
        .and_then(|path| Ok(String::from(path.to_string_lossy().to_owned())))
        .expect("Fuck this?");
}

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
                println!("{i:?}");
                let search_results = dir_idx.search_term(&i.1);
                let search_results: HashMap<String, f64> = search_results
                    .iter()
                    .map(|(key, val)| (canonicalize_path(key), *val))
                    .collect();

                let serialized_output = serde_json::to_string(&search_results)?;
                let response = tiny_http::Response::from_data(serialized_output);

                let cors_header =
                    tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..])
                        .expect("Fuck this!");
                let cors_another_header =
                    tiny_http::Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"*"[..])
                        .expect("Another fuck you!");
                request
                    .respond(
                        response
                            .with_header(cors_another_header)
                            .with_header(cors_header),
                    )
                    .expect("I don't care!");
                break 'inner;
            }
        }
    }
    Ok(())
}
