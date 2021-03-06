extern crate prost_build;

use prost_build::{Method, Service, ServiceGenerator};

pub struct Twirp {
  service_count: u16,
}

impl Twirp {
  pub fn new() -> Twirp {
    Twirp { service_count: 0 }
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
    buf.push_str(&format!(
      "pub struct {}<S: {}+Send+Sync+'static> {{\n",
      self.server_type(s),
      s.name,
    ));
    buf.push_str(&format!("  service_impl: S,\n"));
    buf.push_str("}\n\n");
    buf.push_str(&format!(
      "impl<S: {}+Send+Sync+'static> {}<S> {{",
      s.name,
      self.server_type(s),
    ));
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
    buf.push_str("    let service_impl = ::std::sync::Arc::new(self.service_impl);\n");
    buf.push_str("    ::hyper::rt::run(::futures::future::lazy(move || {\n");
    buf.push_str("      let server = ::hyper::Server::bind(&addr)\n");
    buf.push_str("        .serve(move || {\n");
    buf.push_str("          // TODO: remove this clone if possible?;\n");
    buf.push_str("          let service_impl = service_impl.clone();\n");
    buf.push_str(
      "          ::hyper::service::service_fn(move |req| Self::route(service_impl.clone(), req))\n",
    );
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
        service_impl: ::std::sync::Arc<S>,
        req: ::hyper::Request<::hyper::Body>
      ) -> Box<\
        ::futures::Future<Item = ::hyper::Response<::hyper::Body>, Error = ::hyper::Error\
      > + Send> {{\n",
    ));
    buf.push_str("    let mut response_builder = ::hyper::Response::builder();\n");
    // a temporary measure to make CORS pass until middleware support is
    // added. Middleware can then let the caller implement CORS support
    // however they see fit.
    buf.push_str("    response_builder.header(\"Content-Type\", \"application/json\");\n");
    buf.push_str("    response_builder.header(\"Access-Control-Allow-Origin\", \"*\");\n");
    buf.push_str("    response_builder.header(\"Access-Control-Allow-Methods\", \"DELETE, POST, GET, OPTIONS\");\n");
    buf.push_str("    response_builder.header(\"Access-Control-Allow-Headers\", \"Content-Type, Access-Control-Allow-Headers, Authorization, X-Requested-With\");\n");
    // allow preflight
    buf.push_str("    if req.method() == ::hyper::Method::OPTIONS {\n");
    buf.push_str("      let resp = response_builder\n");
    buf.push_str("        .status(::hyper::StatusCode::OK)\n");
    buf.push_str("        .body(::hyper::Body::from(\"\"))\n");
    buf.push_str("        .unwrap();\n");
    buf.push_str("      return Box::new(::futures::future::ok(resp));\n");
    buf.push_str("    }\n");
    buf.push_str("    if req.method() != ::hyper::Method::POST {\n");
    buf.push_str("      let resp = response_builder\n");
    buf.push_str("        .status(::hyper::StatusCode::METHOD_NOT_ALLOWED)\n");
    buf.push_str("        .body(::hyper::Body::from(\"method not allowed\"))\n");
    buf.push_str("        .unwrap();\n");
    buf.push_str("      return Box::new(::futures::future::ok(resp));\n");
    buf.push_str("    }\n");
    buf.push_str("    // cloning to transfer ownership to the future\n");
    buf.push_str("    let uri = req.uri().clone();\n");
    buf
      .push_str("    let fut = req.into_body().concat2().and_then(move |body: ::hyper::Chunk| {\n");
    buf.push_str("      let path = uri.path();\n");
    buf.push_str("      let json_result = match path {\n");
    for m in s.methods.iter() {
      buf.push_str(&format!(
        "        \"/twirp/{}.{}/{}\" => {{\n",
        s.package, s.name, m.proto_name,
      ));
      buf.push_str(&format!(
        "          let rpc_req = match serde_json::from_slice::<{}>(&body) {{\n",
        m.input_type
      ));
      buf.push_str("            Ok(rpc_req) => rpc_req,\n");
      buf.push_str("            Err(err) => {\n");
      buf.push_str(&format!(
        "              let err_msg = format!(\"error deserializing {}: {{}}\", err);\n",
        m.input_type,
      ));
      buf.push_str("              info!(\"{}\", err_msg);\n");
      buf.push_str("              let err_json = serde_json::to_string(&::prost_yat::Error{\n");
      buf.push_str("                code: ::prost_yat::ErrorCode::Internal,\n");
      buf.push_str("                msg: err_msg,\n");
      buf.push_str("              }).unwrap_or(\"error message failed to serialize, ironic\".to_owned());\n");
      buf.push_str("              let resp = response_builder\n");
      buf.push_str("                .status(::hyper::StatusCode::METHOD_NOT_ALLOWED)\n");
      buf.push_str("                .body(::hyper::Body::from(err_json))\n");
      buf.push_str("                .unwrap();\n");
      buf.push_str("              return ::futures::future::ok(resp);\n");
      buf.push_str("            }\n");
      buf.push_str("          };\n");
      buf.push_str(&format!(
        "          match service_impl.{}(rpc_req) {{
            Ok(rpc_res) => {{
              info!(\"twirp RPC {} responded Ok\");
              serde_json::to_string(&rpc_res)
            }}
            Err(err_res) => {{
              error!(\"twirp RPC {} requested Err({{:?}})\", &err_res);
              serde_json::to_string(&err_res)
            }}
          }}\n",
        m.name, m.name, m.name,
      ));
      buf.push_str("        },\n");
    }
    buf.push_str("        route => {\n");
    buf.push_str("              info!(\"route not found: {}\", route);\n");
    buf.push_str("              let resp = response_builder\n");
    buf.push_str("                .status(::hyper::StatusCode::NOT_FOUND)\n");
    buf.push_str("                .body(::hyper::Body::from(\"route not found\"))\n");
    buf.push_str("                .unwrap();\n");
    buf.push_str("              return ::futures::future::ok(resp);\n");
    buf.push_str("        }\n");
    buf.push_str("      };\n");
    buf.push_str("      let json_resp = match json_result {\n");
    buf.push_str("        Ok(s) => s,\n");
    buf.push_str(
      "        Err(e) => return ::futures::future::ok(\
       ::hyper::Response::new(::hyper::Body::from(\
       format!(\"serialization error: {}\", e)\
       ))),\n",
    );
    buf.push_str("      };\n");
    buf.push_str("      response_builder.status(::hyper::StatusCode::OK);\n");
    buf.push_str(
      "      let response_result = response_builder.body(::hyper::Body::from(json_resp));\n",
    );
    buf.push_str("      let response = response_result.expect(\"bad response\");\n");
    buf.push_str("      ::futures::future::ok(response)\n");
    buf.push_str("    });\n");
    buf.push_str("    Box::new(fut)\n");
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
    self.service_count += 1;
    buf.push_str("\n");
    self.write_trait(&s, buf);
    self.write_server(&s, buf);
  }

  fn finalize(&mut self, buf: &mut String) {
    if self.service_count > 0 {
      buf.push_str("use log::{info, error};");
      buf.push_str("use ::hyper::rt::{Future, Stream};");
    }
  }
}
