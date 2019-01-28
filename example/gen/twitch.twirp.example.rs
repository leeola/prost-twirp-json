/// A Hat is a piece of headwear made by a Haberdasher.
#[derive(Clone, PartialEq, Message)]
#[derive(Deserialize, Serialize)]
pub struct Hat {
    /// The size of a hat should always be in inches.
    #[prost(int32, tag="1")]
    pub size: i32,
    /// The color of a hat will never be 'invisible', but other than
    /// that, anything is fair game.
    #[prost(string, tag="2")]
    pub color: String,
    /// The name of a hat is it's type. Like, 'bowler', or something.
    #[prost(string, tag="3")]
    pub name: String,
}
/// Size is passed when requesting a new hat to be made. It's always
/// measured in inches.
#[derive(Clone, PartialEq, Message)]
#[derive(Deserialize, Serialize)]
pub struct Size {
    #[prost(int32, tag="1")]
    pub inches: i32,
}

/// A Haberdasher makes hats for clients.
pub trait Haberdasher {
    /// MakeHat produces a hat of mysterious, randomly-selected color!
    fn make_hat(&self, r: Size) -> Result<Hat, ::prost_yat::Error>;
}

pub struct HaberdasherServer<S: Haberdasher+Send+Sync+'static> {
  service_impl: S,
}

impl<S: Haberdasher+Send+Sync+'static> HaberdasherServer<S> {
  pub fn new(service_impl: S) -> HaberdasherServer<S> {
    HaberdasherServer {
      service_impl: service_impl,
    }
  }

  pub fn listen(self, addr: ::std::net::SocketAddr) {
    let service_impl = ::std::sync::Arc::new(self.service_impl);
    ::hyper::rt::run(::futures::future::lazy(move || {
      let server = ::hyper::Server::bind(&addr)
        .serve(move || {
          // TODO: remove this clone if possible?;
          let service_impl = service_impl.clone();
          ::hyper::service::service_fn(move |req| Self::route(service_impl.clone(), req))
        })
        .map_err(|e| eprintln!("server error: {}", e));      server
    }));
  }

  fn route(
        service_impl: ::std::sync::Arc<S>,
        req: ::hyper::Request<::hyper::Body>
      ) -> Box<::futures::Future<Item = ::hyper::Response<::hyper::Body>, Error = ::hyper::Error> + Send> {
    let mut response_builder = ::hyper::Response::builder();
    response_builder.header("Content-Type", "application/json");
    response_builder.header("Access-Control-Allow-Origin", "*");
    response_builder.header("Access-Control-Allow-Methods", "DELETE, POST, GET, OPTIONS");
    response_builder.header("Access-Control-Allow-Headers", "Content-Type, Access-Control-Allow-Headers, Authorization, X-Requested-With");
    if req.method() == ::hyper::Method::OPTIONS {
      let resp = response_builder
        .status(::hyper::StatusCode::OK)
        .body(::hyper::Body::from(""))
        .unwrap();
      return Box::new(::futures::future::ok(resp));
    }
    if req.method() != ::hyper::Method::POST {
      let resp = response_builder
        .status(::hyper::StatusCode::METHOD_NOT_ALLOWED)
        .body(::hyper::Body::from("method not allowed"))
        .unwrap();
      return Box::new(::futures::future::ok(resp));
    }
    // cloning to transfer ownership to the future
    let uri = req.uri().clone();
    let fut = req.into_body().concat2().and_then(move |body: ::hyper::Chunk| {
      let path = uri.path();
      let json_result = match path {
        "/twirp/twitch.twirp.example/MakeHat" => {
          let rpc_req = match serde_json::from_slice::<Size>(&body) {
            Ok(rpc_req) => rpc_req,
            Err(err) => {
              let err_msg = format!("error deserializing Size: {}", err);
              info!("{}", err_msg);
              let resp = response_builder
                .status(::hyper::StatusCode::METHOD_NOT_ALLOWED)
                .body(::hyper::Body::from(err_msg))
                .unwrap();
              return ::futures::future::ok(resp);
            }
          };
          match service_impl.make_hat(rpc_req) {
            Ok(rpc_res) => {
              info!("twirp RPC make_hat responded Ok");
              serde_json::to_string(&rpc_res)
            }
            Err(err_res) => {
              error!("twirp RPC make_hat requested Err({:?})", &err_res);
              serde_json::to_string(&err_res)
            }
          }
        },
        route => {
              info!("route not found: {}", route);
              let resp = response_builder
                .status(::hyper::StatusCode::NOT_FOUND)
                .body(::hyper::Body::from("route not found"))
                .unwrap();
              return ::futures::future::ok(resp);
        }
      };
      let json_resp = match json_result {
        Ok(s) => s,
        Err(e) => return ::futures::future::ok(::hyper::Response::new(::hyper::Body::from(format!("serialization error: {}", e)))),
      };
      response_builder.status(::hyper::StatusCode::OK);
      let response_result = response_builder.body(::hyper::Body::from(json_resp));
      let response = response_result.expect("bad response");
      ::futures::future::ok(response)
    });
    Box::new(fut)
  }
}
use log::{info, error};use ::hyper::rt::{Future, Stream};