#[macro_use]
extern crate prost_derive;
extern crate prost_yat;
extern crate hyper;
extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate log;
extern crate env_logger;

use env_logger::Env;
use log::info;
use prost_yat::*;

mod proto {
  include!(concat!(env!("OUT_DIR"), "/twitch.twirp.example.rs"));
}

fn main() {
  let env = Env::new().filter_or("RUST_LOG", "info");
  env_logger::init_from_env(env);

  let s = proto::HaberdasherServer::<Haber>::new(Haber{});
  info!("starting server..");
  s.listen(([127, 0, 0, 1], 3000).into());
}

struct Haber;
impl proto::Haberdasher for Haber {
  fn make_hat(&self, _r: proto::Size) -> Result<proto::Hat, Error> {
    Err("make_hat not implemented".into())
  }
}
