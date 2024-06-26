use crate::sgx::{read_quote, write_user_data};
use crate::ts::run;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Instant;

mod sgx;
mod ts;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello world!".into()))
}

// See sgx.max_threads in the manifest.
#[tokio::main(worker_threads = 4)]
async fn main() {
    println!("Starting...");

    // let h = b"your data here"; // replace with your data
    // write_user_data(h).unwrap();
    // let quote = read_quote().unwrap();
    // println!("quote");
    // for byte in quote {
    //     print!("{:02x}", byte);
    // }
    // println!();

    let start = Instant::now();
    run().await;
    let duration = start.elapsed();
    println!("Time elapsed in run() is: {:?}", duration);

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // let make_service =
    //     make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });
    // let server = Server::bind(&addr).serve(make_service);
    // if let Err(e) = server.await {
    //     eprintln!("server error: {}", e);
    //     std::process::exit(1);
    // }
}
