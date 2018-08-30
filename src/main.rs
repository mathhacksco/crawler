#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

// extern crate rocket;
// #[macro_use]
// extern crate rocket_contrib;
// extern crate rocket_cors;
// #[macro_use]
// extern crate hyper;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate serde_derive;
extern crate argon2rs;
extern crate chrono;
extern crate rand;
extern crate redis;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate uuid;
extern crate validator;

use redis::RedisError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJSONError;

#[derive(Debug)]
pub enum Error {
    RedisError(RedisError),
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

impl From<RedisError> for Error {
    fn from(error: RedisError) -> Error {
        Error::RedisError(error)
    }
}

use redis::Commands;
mod integrations;
use integrations::medium::api::fetch_posts;

fn main() {
    update_cached_posts().unwrap();
}

fn update_cached_posts() -> Result<(), Error> {
    let publication = dotenv!("MEDIUM_PUBLICATION");
    let res = fetch_posts(publication)?;
    let posts = res.payload.references.post;
    let redis = init_redis()?;
    redis.set("MEDIUM_POSTS", serde_json::to_string(&posts)?)?;
    Ok(())
}

// TODO create async interface with http://mitsuhiko.github.io/redis-rs/redis/#async
fn init_redis() -> Result<redis::Connection, Error> {
    let uri = dotenv!("REDIS_URI");
    let client = redis::Client::open(uri)?;
    let conn = client.get_connection()?;
    Ok(conn)
}
