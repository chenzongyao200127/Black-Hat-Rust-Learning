use rayon::prelude::*;
// Rayon is a data-parallelism library for Rust. It is extremely lightweight and makes it easy to convert a sequential 
// computation into a parallel one. It also guarantees data-race freedom. (You may also enjoy this blog post about Rayon, 
// which gives more background and details about how it works, or this video, 
// from the Rust Belt Rust conference.) Rayon is available on crates.io, and API Documentation is available on docs.rs.
use reqwest::{blocking::Client, redirect};
// An ergonomic, batteries-included HTTP Client for Rust.
use std::{env, time::Duration};

mod error;
pub use error::Error;
mod model;
mod ports;
mod subdomains;
use model::Subdomain;
mod common_ports;

/// Our first scanner in Rust
/// We will use the api provided by crt.sh which can be queried by 
/// calling the following endpoint: https://crt.sh/?q=%25.[domain.com]&out

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].as_str();

    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    // we use a custom trheadpool to improve speed
    let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(256)
    .build()
    .unwrap();

    pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();

        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("      {}", port.port);
            }

            println!("")
        }
    });


    Ok(())
}