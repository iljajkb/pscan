use std::net::Ipv4Addr;
use pnet::packet::tcp::{MutableTcpPacket, TcpFlags};
use pnet::transport::{TransportSender, TransportReceiver, transport_channel};
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::Ipv4;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::ipv4_checksum;
use std::collections::HashMap;
use std::time::{Duration};
use std::sync::Arc;
use std::net::UdpSocket;

pub async fn scan_ports_syn(ports: Vec<u16>, target_ip: &String, services: &Arc<HashMap<u16, String>>) {
    let (mut tx, mut rx) = open_channel();

    let source_ip = get_local_ip_for_target(target_ip)
      .expect("Could not determine local IP for routing to target");
        
    println!("Using Source IP: {}", source_ip);

    let target = target_ip.parse().expect("invalid target ip");
    
    let services_clone = Arc::clone(services);

    let listener = tokio::task::spawn_blocking(move || {
      let mut iter = pnet::transport::tcp_packet_iter(&mut rx);

      loop {
        match iter.next() {
          Ok((packet, addr)) => {

            if addr == target {

              let flags = packet.get_flags();
              let sport = packet.get_source();
                // SYN-ACK = Port ist offen
              if flags & (TcpFlags::SYN | TcpFlags::ACK) == (TcpFlags::SYN | TcpFlags::ACK) {
                let service_name = services_clone.get(&sport)
                  .map(|s| s.as_str())
                  .unwrap_or("unknown");
                println!("{}/tcp      open       {}", packet.get_source(), service_name);
              }
            }
          },
          Err(_e) => break,
        }   
      }
    });
    // time for listener to start fully
    tokio::time::sleep(Duration::from_millis(500)).await;

    // FIRE SENDING
    for port in ports {
      build_and_send_tcp_packet(12345, port, source_ip, target, &mut tx);
      tokio::time::sleep(Duration::from_millis(1)).await;
    }

    // Cooldown, all SYN packets send
    tokio::time::sleep(Duration::from_secs(2)).await;

    listener.abort();
}

fn open_channel() -> (TransportSender, TransportReceiver) {

  let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Tcp));
  let (tx, rx) = match transport_channel(4096, protocol) {
    Ok((tx, rx)) => (tx, rx),
    Err(e) => panic!("Error while opening channel for SYN SCAN: {}", e),
  };

  (tx, rx)
}

fn build_and_send_tcp_packet(
    source_port: u16, 
    target_port: u16,
    source_ip: Ipv4Addr,
    target_ip: Ipv4Addr,
    tx: &mut pnet::transport::TransportSender
) {

  let mut packet_buffer = [0u8; 20]; // 20 bytes default tcp header
  let mut tcp_packet = MutableTcpPacket::new(&mut packet_buffer).unwrap();
  
  
  tcp_packet.set_source(source_port);
  tcp_packet.set_destination(target_port);
  tcp_packet.set_sequence(1000);
  tcp_packet.set_data_offset(5);

  // set SYNbit = 1
  tcp_packet.set_flags(TcpFlags::SYN);
  
  let checksum = ipv4_checksum(&tcp_packet.to_immutable(), &source_ip, &target_ip);
    
  
  tcp_packet.set_checksum(checksum);

  match tx.send_to(tcp_packet, std::net::IpAddr::V4(target_ip)) {
    Ok(_) => {},
    Err(e) => println!("Error while sending SYN: {}", e),
  }

}

pub fn get_local_ip_for_target(target_ip: &str) -> Option<std::net::Ipv4Addr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    
    socket.connect(format!("{}:80", target_ip)).ok()?;
    
    let local_addr = socket.local_addr().ok()?;
    
    if let std::net::IpAddr::V4(ipv4) = local_addr.ip() {
        Some(ipv4)
    } else {
        None
    }
}


