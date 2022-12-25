use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use zmq::Context;
use zmq::Socket;

use serde::Serialize;

use crate::utils;

pub struct Publisher {
  publisher: Socket,
}

impl Publisher {
  pub fn new(port: &u16) -> Result<Publisher, Box <dyn Error>> { 
    let context: Context = zmq::Context::new();
    let publisher: Socket = context.socket(zmq::PUB)?;
    let endpoint: String = utils::get_binding_endpoint(port);

    publisher.bind(&endpoint)?;
    sleep(Duration::from_millis(1000));
    
    Ok(Publisher { 
      publisher: publisher,
    })
  }

  pub fn publish<T: Serialize>(&self, topic: &String, data: &T) -> Result<(), Box <dyn Error>> {
    let publish_message: String = serde_json::to_string(&data)?;
    self.publisher.send(topic, zmq::SNDMORE)?;
    self.publisher.send(&publish_message, 0)?;
    Ok(())
  }
}
