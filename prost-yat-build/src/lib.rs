extern crate prost_build;

use prost_build::{Method, Service, ServiceGenerator};

pub struct Twirp;

impl Twirp {
  pub fn new() -> Twirp {
    Twirp {}
  }

  fn write_trait(&self, service: &Service, buf: &mut String) {
    buf.push_str("\n");
    service.comments.append_with_indent(0, buf);
    buf.push_str(&format!("pub trait {} {{", service.name));
    for method in service.methods.iter() {
      buf.push_str("\n");
      method.comments.append_with_indent(1, buf);
      buf.push_str(&format!(
        "    fn {}(&self, r: {}) -> {};\n",
        method.name,
        method.input_type,
        self.return_type(method)
      ));
    }
    buf.push_str("}\n");
  }

  // TODO: sync feature (default)
  fn return_type(&self, m: &Method) -> String {
    format!("Result<{}, ::prost_yat::Error>", m.output_type)
  }
}

impl ServiceGenerator for Twirp {
  fn generate(&mut self, s: Service, buf: &mut String) {
    self.write_trait(&s, buf);
  }
}
