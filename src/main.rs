use std::net::TcpStream;
use std::time::Duration;
use std::net::ToSocketAddrs;


fn main() {
  let host = "google.com";
  let port = "443";


  let timeout = Duration::from_secs(3);
  let target = format!("{}:{}", host, port);
  println!("Target: {}:{}",host, port);

  match target.to_socket_addrs() {
    Ok(mut addrs) => {
      if let Some(socket_addr) = addrs.next() {
        match TcpStream::connect_timeout(&socket_addr, timeout) {
          Ok(_) => println!("{}/tcp offen", port),
          Err(_) => println!("{}/tcp geschlossen", port),
        }
      } else {
        println!("Error: could not find IP for given domain");
      }
    }
    Err(_e) => println!("DNS error!")
  }
}
