# ZeroMQ Wrapper
This library is inteded to provide a ergonomic rust wrapper for the most commom patterns of ZeroMQ, using the powerfull [serde](https://github.com/serde-rs/serde) serialization/deserialization features. Currently, all wrappers are not blocking, returning an `std::Option<T>` for not ready or erroneous values.

## Patterns
Currently there are two patterns available: 
- Request/Response (Or Client/Server)
- Publisher/Subscriber

## Contributing
As I am not like a pro, this code should be improved and I'll appreciate your help. Consider opening an issue if you figure-out a bug, or even a pull-request. 
