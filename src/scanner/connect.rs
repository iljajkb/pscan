use std::net::TcpStream;
use std::time::Duration;
use std::net::ToSocketAddrs;
use std::collections::HashMap;


/// complete 3-way-TCP-handshake
pub fn scan_port_connect(host: &str, port: u16, services: &HashMap<u16, String>) {
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

