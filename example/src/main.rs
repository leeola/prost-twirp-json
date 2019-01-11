#[macro_use]
extern crate prost_derive;
extern crate prost_yat;
extern crate hyper;
extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use prost_yat::*;

mod proto {
  include!(concat!(env!("OUT_DIR"), "/twitch.twirp.example.rs"));
}

fn main() {
  let s = proto::HaberdasherServer::<Haber>::new(Haber{});
  println!("starting server..");
  s.listen(([127, 0, 0, 1], 3000).into());
}

struct Haber;
impl proto::Haberdasher for Haber {
  fn make_hat(&self, r: proto::Size) -> Result<proto::Hat, Error> {
    Err("make_hat not implemented".into())
  }
}
