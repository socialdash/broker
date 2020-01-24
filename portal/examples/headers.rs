#![deny(warnings)]
use std::net::SocketAddr;
use portal::Filter;

/// Create a server that requires header conditions:
///
/// - `Host` is a `SocketAddr`
/// - `Accept` is exactly `*/*`
///
/// If these conditions don't match, a 404 is returned.
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // For this example, we assume no DNS was used,
    // so the Host header should be an address.
    let host = portal::header::<SocketAddr>("host");

    // Match when we get `accept: */*` exactly.
    let accept_stars = portal::header::exact("accept", "*/*");

    let routes = host
        .and(accept_stars)
        .map(|addr| format!("accepting stars on {}", addr));

    portal::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
