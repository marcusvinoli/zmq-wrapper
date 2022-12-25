use zmq_wrapper::requester::Requester;
use zmq_wrapper::responder::Responder;

use std::thread;

const TEST_PORT: u16 = 8089;

#[test]
fn req_res_test() {
  let responder: Responder = Responder::new(&TEST_PORT).unwrap();
  let requester: Requester = Requester::new(&TEST_PORT).unwrap();
  let mut response: Option<String> = None;
  let mut ghost_response: Option<isize> = None;
  let hello:String = String::from("Hello");

  thread::spawn(move || {
    loop {
      let path: String = responder.path().unwrap();
      if path == String::from("Hi, mom") {
        responder.send(&hello).unwrap();
      } else if path == String::from("Generic/topic") {
        responder.send(&10).unwrap();
      }
    }
  });

  ghost_response = requester.request("Generic/topic");
  response = requester.request("Hi, mom");

  assert_eq!(response.unwrap(), "Hello");
  assert_eq!(ghost_response.unwrap(), 10);
}