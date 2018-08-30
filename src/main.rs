#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

// extern crate rocket;
// #[macro_use]
// extern crate rocket_contrib;
// extern crate rocket_cors;
extern crate dotenv;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate argon2rs;
extern crate rand;
// #[macro_use]
// extern crate hyper;
extern crate reqwest;
extern crate validator;
extern crate uuid;
extern crate url;

use std::env;
use serde_json::Error as SerdeJSONError;
use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum Error {
  SerdeJSONError(SerdeJSONError),
  ReqwestError(ReqwestError),
  StringError(String),
}

impl From<SerdeJSONError> for Error {
  fn from(error: SerdeJSONError) -> Error {
    Error::SerdeJSONError(error)
  }
}

impl From<ReqwestError> for Error {
  fn from(error: ReqwestError) -> Error {
    Error::ReqwestError(error)
  }
}

mod integrations;

use dotenv::dotenv;
use integrations::medium::api::fetch_posts;

fn main() {
  dotenv().ok();
  let publication = env::var("MEDIUM_PUBLICATION").expect("Missing required parameter \"MEDIUM_PUBLICATION\"");
  let posts = fetch_posts(publication);
  println!("{:?}", posts);
}
