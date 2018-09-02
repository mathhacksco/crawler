#![feature(plugin)]
#![feature(custom_derive)]
#![feature(extern_prelude)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate serde_derive;
extern crate redis;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use redis::RedisError;
use reqwest::Error as ReqwestError;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket::Request;
use serde::{Serialize, Serializer};
use serde_json::Error as SerdeJSONError;
use std::io::Cursor;

#[derive(Debug)]
pub enum Error {
    RedisError(RedisError),
    SerdeJSONError(SerdeJSONError),
    ReqwestError(ReqwestError),
    StringError(String),
    RouteError(ErrorResponse),
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

#[derive(Debug)]
pub enum ErrorResponse {
    NotFound(ErrorResponseData),
    InternalServerError(ErrorResponseData),
    Unauthorized(ErrorResponseData),
    ServiceUnavailable(ErrorResponseData),
}

impl ErrorResponse {
    pub fn as_status(&self) -> Status {
        match *self {
            ErrorResponse::NotFound(_) => Status::NotFound,
            ErrorResponse::InternalServerError(_) => Status::InternalServerError,
            ErrorResponse::Unauthorized(_) => Status::Unauthorized,
            ErrorResponse::ServiceUnavailable(_) => Status::ServiceUnavailable,
        }
    }

    pub fn not_found(uri: String) -> ErrorResponse {
        ErrorResponse::NotFound(ErrorResponseData {
            status: 404,
            message: "Not Found",
            uri,
        })
    }

    pub fn internal_server_error(uri: String) -> ErrorResponse {
        ErrorResponse::InternalServerError(ErrorResponseData {
            status: 500,
            message: "Internal Server Error",
            uri,
        })
    }

    pub fn bad_request(uri: String) -> ErrorResponse {
        ErrorResponse::InternalServerError(ErrorResponseData {
            status: 400,
            message: "Bad Request",
            uri,
        })
    }

    pub fn unauthorized(uri: String) -> ErrorResponse {
        ErrorResponse::Unauthorized(ErrorResponseData {
            status: 401,
            message: "Unauthorized",
            uri,
        })
    }

    pub fn service_unavailable(uri: String) -> ErrorResponse {
        ErrorResponse::ServiceUnavailable(ErrorResponseData {
            status: 503,
            message: "Service Unavailable",
            uri,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct ErrorResponseData {
    status: u16,
    message: &'static str,
    uri: String,
}

impl<'a> Responder<'a> for Error {
    fn respond_to(self, _: &Request) -> Result<Response<'a>, Status> {
        eprintln!("{:?}", self);
        match self {
            Error::RouteError(error) => {
                let json = serde_json::to_string(&error).unwrap();
                Ok(Response::build()
                    .status(error.as_status())
                    .header(ContentType::JSON)
                    .sized_body(Cursor::new(json))
                    .finalize())
            }
            _ => Err(Status::InternalServerError),
        }
    }
}

impl Serialize for ErrorResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ErrorResponse::NotFound(ref data) => data.serialize(serializer),
            ErrorResponse::InternalServerError(ref data) => data.serialize(serializer),
            ErrorResponse::Unauthorized(ref data) => data.serialize(serializer),
            ErrorResponse::ServiceUnavailable(ref data) => data.serialize(serializer),
        }
    }
}

impl<'a> Responder<'a> for ErrorResponse {
    fn respond_to(self, _: &Request) -> Result<Response<'a>, Status> {
        let json = serde_json::to_string(&self).unwrap();
        Ok(Response::build()
            .status(self.as_status())
            .header(ContentType::JSON)
            .sized_body(Cursor::new(json))
            .finalize())
    }
}

mod integrations;
mod models;
mod routes;
mod server;

use server::init_server;

fn main() {
    let redis_conn = init_redis().expect("Failed to connect to redis");
    init_server(redis_conn);
}

// TODO create async interface with http://mitsuhiko.github.io/redis-rs/redis/#async
fn init_redis() -> Result<redis::Connection, Error> {
    let uri = dotenv!("REDIS_URI");
    let client = redis::Client::open(uri)?;
    let conn = client.get_connection()?;
    Ok(conn)
}
