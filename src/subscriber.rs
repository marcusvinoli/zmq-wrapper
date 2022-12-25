use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use zmq::Context;
use zmq::Socket;
use zmq::PollItem;

use serde::Deserialize;

use crate::utils;
pub struct Subscriber {
  subscriber: Socket,
  topic: String
}

impl Subscriber {
  pub fn new(port: &u16, topic: &String) -> Result<Subscriber, Box <dyn Error>> { 
    let context: Context = zmq::Context::new();
    let sub: Socket = context.socket(zmq::SUB)?;
    let endpoint: String = utils::get_connection_endpoint(port);

    sub.connect(&endpoint)?;
    sleep(Duration::from_millis(1000));

    sub.set_subscribe(topic.as_bytes())?;
    
    Ok(Subscriber { 
      subscriber: sub,
      topic: topic.clone(),
    })
  }

  pub fn poll<T: for<'a> Deserialize<'a>>(&self) -> Option<T> {
    let mut poll_item: [PollItem; 1] = [self.subscriber.as_poll_item(zmq::POLLIN)];
    if zmq::poll(&mut poll_item, 1000).is_err() {
      return None;
    }
    if poll_item[0].is_readable() {
      let topic = self.subscriber.recv_string(0).expect("Error parsing Topic").unwrap();
      let data: String = self.subscriber.recv_string(0).expect(&format!("Error receiving Data for topic {}", self.topic)).unwrap();
      if topic == self.topic {
        if let Ok(value) = serde_json::from_str::<T>(data.as_str()) {
          return Some(value);
        }
      }
    }
    return None;
  }
}
