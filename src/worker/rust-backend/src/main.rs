mod ioio;
use ioio::start;
use std::thread;

fn main() {
    thread::spawn(move || start("50051").expect("Rpc client error!"));
}
