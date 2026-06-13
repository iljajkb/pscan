use clap::{Parser, ValueEnum};

/// small and fast port scanner in rust, by iljaj
#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
pub struct Args {
  pub host: String,
  #[arg(short, long, default_value = "1-1000")]
  pub ports: String,
  /// Scan type to use
  #[arg(short, long, value_enum, default_value_t = ScanType::Connect)]
  pub type_scan: ScanType,

  /// Speed/Concurrency (number of parallel requests
  #[arg(short, long, default_value_t = 200)]
  pub speed: usize,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScanType {
    /// Full 3-Way Handshake (TCP Connect)
    Connect,
    /// TCP SYN Scan (Stealth) - Requires root privileges
    Syn,
}

pub fn parse_ports(port_arg: &str) -> Vec<u16> {
  if port_arg == "-" {
    return (1..=65535).collect();
  }

  if port_arg.contains('-') {
    let parts: Vec<&str> = port_arg.split('-').collect();
      if parts.len() == 2 {
        if let (Ok(start), Ok(end)) = (parts[0].parse::<u16>(), parts[1].parse::<u16>()) {
          if start <= end {
            return (start..=end).collect();
        }
      }
    }
  }

  if let Ok(single_port) = port_arg.parse::<u16>() {
    return vec![single_port];
  }

  println!("[-] invalid port format - scanning standard ports 80,443");
  vec![80,443]

}
