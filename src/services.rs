use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServiceInfo {
    pub description: String,
}

pub fn load_services() -> HashMap<u16, String> {
    let json_data = include_str!("resources/ports.lists.json");

    let raw_map: HashMap<String, Vec<ServiceInfo>> = serde_json::from_str(json_data)
        .unwrap_or_else(|_| HashMap::new());

    let mut service_map = HashMap::new();
    for (port_str, list_of_services) in raw_map {
        if let Ok(port) = port_str.parse::<u16>() {
          if let Some(first_service) = list_of_services.first() {
            service_map.insert(port, first_service.description.clone());
          }
        }
    }

    service_map
}
