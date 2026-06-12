mod cli;
mod services;
mod scanner;

use clap::Parser;

fn main() {
  let args = cli::Args::parse();


  let host = args.host;
  let ports = cli::parse_ports(&args.ports);
 
  let service_map = services::load_services();
  println!("Scanning: {} ports for {}",ports.len(), host);

  println!("PORT      STATE      SERVICE");

  for &port in &ports {
    scanner::scan_port_connect(&host, port, &service_map);
  }  

  println!("Scan completed!")
}
