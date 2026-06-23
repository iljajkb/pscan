use tokio::net::TcpStream;
use std::time::Duration;
use tokio::time::timeout;
use std::collections::HashMap;
use std::sync::Arc;

/// complete 3-way-TCP-handshake
pub async fn scan_port_connect(host: &str, port: u16, services: &Arc<HashMap<u16, String>>) {
  let target = format!("{}:{}", host, port);
  
  let timeout_duration = Duration::from_millis(200);

  let services_clone = Arc::clone(services);

  let connection_attempt = timeout(
    timeout_duration, 
    TcpStream::connect(&target)
  ).await;

  if let Ok(Ok(_stream)) = connection_attempt {
    let service_name = services_clone.get(&port)
      .map(|s| s.as_str())
      .unwrap_or("unknown");
    println!("{}/tcp      open      {}", port, service_name);
  }
}
