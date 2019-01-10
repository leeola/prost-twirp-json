#[macro_use]
extern crate prost_derive;
extern crate prost_yat;

use prost_yat::*;

mod proto {
  include!(concat!(env!("OUT_DIR"), "/twitch.twirp.example.rs"));
}

fn main() {
  let s = proto::HaberdasherServer::<Haber>::new(Haber{});
  println!("Hello, world!");
}

struct Haber;

impl proto::Haberdasher for Haber {
  fn make_hat(&self, r: proto::Size) -> Result<proto::Hat, Error> {
    Err("foo".into())
  }
}
