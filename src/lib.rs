#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::convert::From;

#[derive(Serialize, Deserialize)]
pub struct Error {
  code: ErrorCode,
  msg: String,
  // TODO: add some type of generic key/value for arbitrary
  // metadata inclusion.
  // meta: Option<HashMap>,
}

#[derive(Serialize, Deserialize)]
pub enum ErrorCode {
  Canceled,
  Unknown,
  InvalidArgument,
  DeadlineExceeded,
  NotFound,
  BadRoute,
  AlreadyExists,
  PermissionDenied,
  Unauthenticated,
  ResourceExhausted,
  FailedPrecondition,
  Aborted,
  OutOfRange,
  Unimplemented,
  Internal,
  Unavailable,
  Dataloss,
}

impl ErrorCode {
  /// Returns the http status code corresponding to the Twirp error code.
  ///
  /// As defined by:
  ///   https://twitchtv.github.io/twirp/docs/spec_v5.html#error-codes
  pub fn to_http_status_code(&self) -> i16 {
    match self {
      Canceled => 408,
      Unknown => 500,
      InvalidArgument => 400,
      DeadlineExceeded => 408,
      NotFound => 404,
      BadRoute => 404,
      AlreadyExists => 409,
      PermissionDenied => 403,
      Unauthenticated => 401,
      ResourceExhausted => 403,
      FailedPrecondition => 412,
      Aborted => 409,
      OutOfRange => 400,
      Unimplemented => 501,
      Internal => 500,
      Unavailable => 503,
      Dataloss => 500,
    }
  }
}

impl From<String> for Error {
  fn from(s: String) -> Self {
    return Error{
      code: ErrorCode::Internal,
      msg: s,
    }
  }
}

impl From<&'static str> for Error {
  fn from(s: &'static str) -> Self {
    return Error{
      code: ErrorCode::Internal,
      msg: String::from(s),
    }
  }
}
