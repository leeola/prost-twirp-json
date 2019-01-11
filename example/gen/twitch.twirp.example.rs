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
    if req.method() != ::hyper::Method::POST {
      return Box::new(::futures::future::ok(::hyper::Response::new(::hyper::Body::from("method not found"))));
    }
    // cloning to transfer ownership to the future
    let uri = req.uri().clone();
    let fut = req.into_body().concat2().and_then(move |body: ::hyper::Chunk| {
      let json_result = match uri.path() {
        "/twirp/twitch.twirp.example/MakeHat" => {
          let rpc_req = match serde_json::from_slice::<Size>(&body) {
            Ok(rpc_req) => rpc_req,
            Err(e) => return ::futures::future::ok(::hyper::Response::new(::hyper::Body::from(format!("error deserializing Size: {}", e)))),
          };
          match service_impl.make_hat(rpc_req) {
            Ok(rpc_res) => serde_json::to_string(&rpc_res),
            Err(err_res) => serde_json::to_string(&err_res),
          }
      },
        _ => return ::futures::future::ok(::hyper::Response::new(::hyper::Body::from("route not found"))),
      };
      let json_resp = match json_result {
        Ok(s) => s,
        Err(e) => return ::futures::future::ok(::hyper::Response::new(::hyper::Body::from(format!("serialization error: {}", e)))),
      };
      ::futures::future::ok(::hyper::Response::new(::hyper::Body::from(json_resp)))
    });
    Box::new(fut)
  }
}
use ::hyper::rt::{Future, Stream};