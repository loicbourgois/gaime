extern crate ws;

use ws::{connect, CloseCode};

fn main() {
  connect("ws://0.0.0.0:8080", |out| {
      out.send("loginbob").unwrap();

      move |msg| {
          println!("Got message: {}", msg);
          out.close(CloseCode::Normal)
      }
  }).unwrap()
}
