#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::convert::From;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
  pub code: ErrorCode,
  pub msg: String,
  // TODO: add some type of generic key/value for arbitrary
  // metadata inclusion.
  // pub meta: Option<HashMap>,
}

#[derive(Debug, Serialize, Deserialize)]
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
      ErrorCode::Canceled => 408,
      ErrorCode::Unknown => 500,
      ErrorCode::InvalidArgument => 400,
      ErrorCode::DeadlineExceeded => 408,
      ErrorCode::NotFound => 404,
      ErrorCode::BadRoute => 404,
      ErrorCode::AlreadyExists => 409,
      ErrorCode::PermissionDenied => 403,
      ErrorCode::Unauthenticated => 401,
      ErrorCode::ResourceExhausted => 403,
      ErrorCode::FailedPrecondition => 412,
      ErrorCode::Aborted => 409,
      ErrorCode::OutOfRange => 400,
      ErrorCode::Unimplemented => 501,
      ErrorCode::Internal => 500,
      ErrorCode::Unavailable => 503,
      ErrorCode::Dataloss => 500,
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
