pub fn get_binding_endpoint(port: &u16) -> String {
  let mut endpoint: String = String::from("tcp://*:");
  endpoint.push_str(&port.to_string());
  return endpoint;
}

pub fn get_connection_endpoint(port: &u16) -> String {
  let mut endpoint: String = String::from("tcp://localhost:");
  endpoint.push_str(&port.to_string());
  endpoint
}
