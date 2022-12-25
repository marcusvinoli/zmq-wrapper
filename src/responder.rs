use std::error::Error;

use zmq::Context;
use zmq::Message;
use zmq::Socket;
use zmq::PollItem;

use serde::Serialize;

use crate::utils;

pub struct Responder {
  socket: Socket,
}

impl Responder {
  pub fn new(port: &u16) -> Result<Responder, Box <dyn Error>> {
    let context: Context = Context::new();
    let socket: Socket = context.socket(zmq::REP)?;
    let endpoint: String = utils::get_binding_endpoint(&port);
    socket.bind(&endpoint)?;
    
    Ok(Responder {
      socket: socket,
    })
  }

  pub fn path(&self) -> Result<String, Box <dyn Error>> {
    let mut ret_string: String = String::new();
    let mut msg: Message = Message::new();
    let poll_item: PollItem = self.socket.as_poll_item(zmq::POLLIN);
    let mut poll_items: [PollItem; 1] = [poll_item];
    if let Ok(n) = zmq::poll(&mut poll_items, 1000) {
      if n > 0 {
        self.socket.recv(&mut msg, 0)?;
        if let Some(string_msg) = msg.as_str() {
          ret_string = String::from(string_msg);
      }
    }
    }
    return Ok(ret_string);
  }

  pub fn send<T: Serialize>(&self, data: &T) -> Result<(), Box <dyn Error>>{
    let data_string: String = serde_json::to_string(data)?;
    self.socket.send(&data_string, 0)?;
    Ok(())
  }
}
