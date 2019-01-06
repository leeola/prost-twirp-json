extern crate prost_build;

use prost_build::{Service, ServiceGenerator};

pub struct Twirp;

impl Twirp {
  pub fn new() -> Twirp {
    Twirp{}
  }
}

impl ServiceGenerator for Twirp {
  fn generate(&mut self, s: Service, buf: &mut String) {
  }
}
