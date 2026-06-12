use std::net::TcpStream;
use std::time::Duration;
use std::net::ToSocketAddrs;

mod cli;

use clap::Parser;

fn scan_single_port_3_way(host: &str, port: u16) {
  let target = format!("{}:{}", host, port);

  if let Ok(mut addresses) = target.to_socket_addrs() {
    if let Some(ip_addr) = addresses.next() {
      let timeout = Duration::from_millis(200);
      if TcpStream::connect_timeout(&ip_addr, timeout).is_ok() {
        println!("{}/tcp open", port)    
      } 
    }
  }
}

fn main() {
  let args = cli::Args::parse();


  let host = args.host;
  let ports = cli::parse_ports(&args.ports);
 
  println!("Scanning: {} ports for {}",ports.len(), host);

  for &port in &ports {
    scan_single_port_3_way(&host, port);
  }  

  println!("Scan completed!")
}
