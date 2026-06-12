use std::net::TcpStream;
use std::time::Duration;
use std::net::ToSocketAddrs;
use std::collections::HashMap;

mod cli;
mod services;

use clap::Parser;

fn scan_single_port_3_way(host: &str, port: u16, services: &HashMap<u16, String>) {
  let target = format!("{}:{}", host, port);

  if let Ok(mut addresses) = target.to_socket_addrs() {
    if let Some(ip_addr) = addresses.next() {
      let timeout = Duration::from_millis(200);
      if TcpStream::connect_timeout(&ip_addr, timeout).is_ok() {
        let service_name = services.get(&port)
          .map(|s| s.as_str())
          .unwrap_or("unknown");
        println!("{}/tcp      open      {}", port, service_name);    
      } 
    }
  }
}

fn main() {
  let args = cli::Args::parse();


  let host = args.host;
  let ports = cli::parse_ports(&args.ports);
 
  let service_map = services::load_services();
  println!("Scanning: {} ports for {}",ports.len(), host);

  println!("PORT      STATE      SERVICE");

  for &port in &ports {
    scan_single_port_3_way(&host, port, &service_map);
  }  

  println!("Scan completed!")
}
