extern crate prost_build;
extern crate prost_twirp_json;

fn main() {
    let mut conf = prost_build::Config::new();
    conf.service_generator(Box::new(prost_twirp_json::Twirp::new()));
    conf.compile_protos(&["proto/example.proto"], &["proto/"]).unwrap();
}
