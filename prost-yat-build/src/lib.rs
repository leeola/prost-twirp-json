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
    buf.push_str(&format!("pub struct {}<S: Haberdasher+Send+Sync+'static> {{\n", self.server_type(s)));
    buf.push_str(&format!("  service_impl: S,\n"));
    buf.push_str("}\n\n");
    buf.push_str(&format!("impl<S: Haberdasher+Send+Sync+'static> {}<S> {{", self.server_type(s)));
    self.write_func_new(s, buf);
    self.write_func_listen(s, buf);
    self.write_func_route(s, buf);
    buf.push_str("}\n");
  }

  fn write_func_new(&self, s: &Service, buf: &mut String) {
    buf.push_str("\n");
    buf.push_str(&format!(
      "  pub fn new(service_impl: S) -> {}<S> {{\n",
      self.server_type(s),
    ));
    buf.push_str(&format!("    {} {{\n", self.server_type(s)));
    buf.push_str("      service_impl: service_impl,\n");
    buf.push_str("    }\n");
    buf.push_str("  }\n");
  }

  fn write_func_listen(&self, _s: &Service, buf: &mut String) {
    buf.push_str("\n");
    // TODO(leeola): perhaps return a future from this or a related func?
    // to allow for handling errors by the API caller. For now though,
    // it's being handled ignorantly.

    // moving self into listen, so it can be moved into the closure.
    buf.push_str("  pub fn listen(self, addr: ::std::net::SocketAddr) {\n");
    buf.push_str("    use ::futures::Future;\n");
    buf.push_str("    let service_impl = ::std::sync::Arc::new(self.service_impl);\n");
    buf.push_str("    ::hyper::rt::run(::futures::future::lazy(move || {\n");
    buf.push_str("      let server = ::hyper::Server::bind(&addr)\n");
    buf.push_str("        .serve(move || {\n");
    buf.push_str("          // TODO: remove this clone if possible?;\n");
    buf.push_str("          let service_impl = service_impl.clone();\n");
    buf.push_str("          ::hyper::service::service_fn(move |req| Self::route(service_impl.clone(), req))\n");
    buf.push_str("        })\n");
    buf.push_str("        .map_err(|e| eprintln!(\"server error: {}\", e));");
    buf.push_str("      server\n");
    buf.push_str("    }));\n");
    buf.push_str("  }\n");
  }

  fn write_func_route(&self, s: &Service, buf: &mut String) {
    buf.push_str("\n");
    // TODO(leeola): perhaps return a future from this or a related func?
    // to allow for handling errors by the API caller. For now though,
    // it's being handled ignorantly.

    // moving self into listen, so it can be moved into the closure.
    buf.push_str(&format!(
      "  fn route(
        service_impl: ::std::sync::Arc<{}>,
        req: ::hyper::Request<::hyper::Body>
      ) -> Box<\
        ::futures::Future<Item = ::hyper::Response<::hyper::Body>, Error = hyper::Error\
      > + Send> {{\n",
      s.name,
    ));
    buf.push_str("    if req.method() != ::hyper::Method::POST { \n");
    buf.push_str(
      "      return Box::new(::futures::future::ok(\
        ::hyper::Response::new(::hyper::Body::from(\"method not found\"))\
       ));\n",
    );
    buf.push_str("    }\n");
    buf.push_str("    match req.uri().path() {\n");
    for m in s.methods.iter() {
     buf.push_str(&format!(
      "      \"/twirp/{}/{}\" => return Box::new(::futures::future::ok(\
        ::hyper::Response::new(::hyper::Body::from(\"rpc call not implemented\"))\
       )),\n",
       s.package,
       m.proto_name,
    ));
    }
    buf.push_str(
      "      _ => return Box::new(::futures::future::ok(\
        ::hyper::Response::new(::hyper::Body::from(\"route not found\"))\
       )),\n",
    );
    buf.push_str("    }\n");
    buf.push_str(
      "    Box::new(::futures::future::ok(\
        ::hyper::Response::new(::hyper::Body::from(\"not implemented\"))\
       ))\n",
    );
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
