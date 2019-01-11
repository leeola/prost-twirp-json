extern crate prost_build;
extern crate prost_yat_build;

fn main() {
    let mut conf = prost_build::Config::new();
    conf.type_attribute(".", "#[derive(Deserialize, Serialize)]");
    conf.service_generator(Box::new(prost_yat_build::Twirp::new()));
    conf.compile_protos(&["proto/example.proto"], &["proto/"]).unwrap();

}
