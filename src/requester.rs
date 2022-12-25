use std::error::Error;

use zmq::Context;
use zmq::Socket;

use serde::Deserialize;

use crate::utils;

pub struct Requester {
  socket: Socket,
} 

impl Requester {
  pub fn new(port: &u16) -> Result<Requester, Box <dyn Error>> {
    let context: Context = Context::new();
    let socket: Socket = context.socket(zmq::REQ)?;
    let endpoint: String = utils::get_connection_endpoint(&port);
    socket.connect(&endpoint)?;
    
    Ok(Requester {
      socket: socket,
    })
  }

  pub fn request<T: for<'a> Deserialize<'a>>(&self, req: &str) -> Option<T> {
    if self.socket.send(req, 0).is_ok() {
      if let Ok(ok_resp) = self.socket.recv_string(0) {
        if let Ok(resp) = ok_resp {
          let return_value: Option<T> = serde_json::from_str(&resp).ok();
          return return_value;
        }
      }
    }
    return None;
  }
}
