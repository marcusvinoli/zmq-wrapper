use zmq_wrapper::publisher::Publisher;
use zmq_wrapper::subscriber::Subscriber;

const TEST_PORT: u16 = 8008;
const TEST_TOPIC: &str = "zmq/topic";

#[test]
fn pub_sub_test() {
  let test_topic: String = String::from(TEST_TOPIC);
  let subscriber: Subscriber = Subscriber::new(&TEST_PORT, &test_topic).unwrap();
  let publisher: Publisher = Publisher::new(&TEST_PORT).unwrap();
  
  let data_out: Option<isize> = Some(10293);
  publisher.publish(&test_topic, &data_out).unwrap();
  let data_in: Option<isize> = subscriber.poll();
  assert_eq!(data_in, data_out);

  let data_out: Option<isize> = None;
  publisher.publish(&test_topic, &data_out).unwrap();
  let data_in: Option<isize> = subscriber.poll();
  assert_eq!(data_in, data_out);
}
