//! Default Compute@Edge template program.

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

// use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use chrono::{DateTime, SecondsFormat, Utc};

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // let now = SystemTime::now();
    // let current_time = now.duration_since(UNIX_EPOCH).unwrap();
    let now: DateTime<Utc> = Utc::now();

    let mut resp = Response::new()
        .with_header("Access-Control-Expose-Headers", "Server,Content-Length,Date")
        .with_header("Access-Control-Allow-Headers", "origin,accept-encoding,referer")
        .with_header("Access-Control-Allow-Methods", "GET,HEAD,OPTIONS")
        .with_header("Access-Control-Allow-Origin", "*")
        .with_header("Cache-Control", "max-age=0, no-cache, no-store")
        .with_header("Content-Type", "text/plain; charset=ISO-8859-1")
        .with_header("Timing-Allow-Origin", "*")
        .with_status(StatusCode::OK);


    match req.get_method() {
        // Allow GET and HEAD requests.
        &Method::GET | &Method::HEAD | &Method::OPTIONS=> (),

        // Deny anything else.
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    // let qs: HashMap<String, String> = req.get_query().unwrap();
    // let qs_vec = req.get_query_str();
    // let mut qs: Vec<&str>;
    let qs: Vec<&str> = match req.get_query_str() {
        Some(q) => q.split("&").collect(),
        None => vec![]
    };

    if qs.contains(&"iso") {
        if qs.contains(&"ms") {
            resp.set_body_text_plain(now.to_rfc3339_opts(SecondsFormat::Millis, true).as_str());
        } else {
            resp.set_body_text_plain(now.to_rfc3339_opts(SecondsFormat::Secs, true).as_str());
        }
    } else if req.get_path() == "/" && req.get_method() == "GET" {
        if qs.contains(&"ms") {
            let millis = now.timestamp_millis() as f64 / 1000.0;
            resp.set_body_text_plain(format!("{}", millis.to_string()).as_str());
        } else {
            resp.set_body_text_plain(format!("{}", now.timestamp().to_string()).as_str());
        }
    } else if req.get_method() == "OPTIONS" {
        resp.remove_header("Cache-Control");
    } else {
        resp.set_status(StatusCode::NOT_FOUND);
    }

    Ok (resp)
}
