extern crate prost_build;

use prost_build::{Method, Service, ServiceGenerator};

pub struct Twirp;

impl Twirp {
  pub fn new() -> Twirp {
    Twirp {}
  }

  fn write_trait(&self, service: &Service, buf: &mut String) {
    service.comments.append_with_indent(0, buf);
    buf.push_str(&format!("pub trait {} {{", service.name));
    for method in service.methods.iter() {
      buf.push_str("\n");
      method.comments.append_with_indent(1, buf);
      buf.push_str(&format!(
        "    fn {}(&self, r: {}) -> {};\n",
        method.name,
        method.input_type,
        self.rpc_return_type(method)
      ));
    }
    buf.push_str("}\n\n");
  }

  fn write_server(&self, s: &Service, buf: &mut String) {
    buf.push_str(&format!("pub struct {} {{\n", self.server_type(s)));
    buf.push_str(&format!("  service_impl: {},\n", s.name));
    buf.push_str("}\n\n");
    buf.push_str(&format!("impl {}{{\n", self.server_type(s)));
    self.write_func_new(s, buf);
    buf.push_str("}\n");
  }

  fn write_func_new(&self, s: &Service, buf: &mut String) {
    buf.push_str(&format!("  pub fn new(service_impl: {}) -> {} {{\n", s.name, self.server_type(s)));
    buf.push_str(&format!("    {} {{\n", s.name));
    buf.push_str("      service_impl: service_impl,\n");
    buf.push_str("    }\n");
    buf.push_str("  }\n");
  }

  // TODO: sync feature (default)
  fn rpc_return_type(&self, m: &Method) -> String {
    format!("Result<{}, ::prost_yat::Error>", m.output_type)
  }

  fn server_type(&self, s: &Service) -> String {
    format!("{}Server", s.name)
  }
}

impl ServiceGenerator for Twirp {
  fn generate(&mut self, s: Service, buf: &mut String) {
    buf.push_str("\n");
    self.write_trait(&s, buf);
    self.write_server(&s, buf);
  }
}
