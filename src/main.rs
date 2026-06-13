mod cli;
mod services;
mod scanner;

use std::time::Instant;
use clap::Parser;
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
  let args = cli::Args::parse();


  let host = args.host;
  let ports_vec = cli::parse_ports(&args.ports);
  let concurrency = args.speed;
 
  let service_map = services::load_services();
  println!("Scanning: {} ports for {}",ports_vec.len(), host);

  println!("PORT      STATE      SERVICE");

  let start_time = Instant::now(); // stopwatch
  let ports_stream = stream::iter(ports_vec);
  ports_stream.for_each_concurrent(concurrency, |port| {
    let host = host.clone();
    let service_map = service_map.clone();

    async move {
      scanner::scan_port_connect(&host, port, &service_map).await;
    }
  }).await;

  let duration = start_time.elapsed();

  println!("--------------------------------------");
  println!("Scan completed in {:.2?}", duration);

}
