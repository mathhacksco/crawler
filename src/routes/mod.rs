use redis::Commands;
use rocket::http::uri::URI;
use rocket::http::Status;
use rocket::{Catcher, Request, Route, State};
use rocket::response::Response;
use rocket_contrib::Json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use integrations::medium::api::fetch_posts;
use {Error, ErrorResponse};

static MEDIUM_PUBLICATION: &'static str = dotenv!("MEDIUM_PUBLICATION");

pub fn init_routes() -> Vec<Route> {
    let mut v = Vec::new();
    v.extend(init_medium_posts_routes());
    v
}

pub fn init_medium_posts_routes() -> Vec<Route> {
    routes![get_posts, update_posts]
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostsPayload {
    posts: Vec<Post>,
}

/**
 * Retrieve a list of medium posts from cache
 */
#[get("/posts")]
fn get_posts<'a>(
    uri: &'a URI<'a>,
    redis_conn_mutex: State<Arc<Mutex<redis::Connection>>>,
) -> Result<Json<PostsPayload>, Error> {
    match redis_conn_mutex.lock() {
        Ok(redis_conn) => {
            let posts_str: String = redis_conn.get("@mathhacks:medium_posts")?;
            let posts = serde_json::from_str::<HashMap<String, Post>>(&*posts_str)?;
            Ok(Json(PostsPayload {
                posts: posts.values().cloned().collect(),
            }))
        }
        Err(_) => Err(Error::RouteError(ErrorResponse::service_unavailable(
            uri.as_str().into(),
        ))),
    }
}

/**
 * Refresh the cache of medium posts
 */
#[post("/posts")]
fn update_posts<'a>(
    uri: &'a URI<'a>,
    redis_conn_mutex: State<Arc<Mutex<redis::Connection>>>,
) -> Result<Response<'a>, Error> {
    match redis_conn_mutex.lock() {
        Ok(redis_conn) => {
            let res = fetch_posts(MEDIUM_PUBLICATION)?;
            let posts = res.payload.references.post;
            redis_conn.set("@mathhacks:medium_posts", serde_json::to_string(&posts)?)?;
            Ok(Response::build()
                .status(Status::NoContent)
                .finalize())
        }
        Err(_) => Err(Error::RouteError(ErrorResponse::service_unavailable(
            uri.as_str().into(),
        ))),
    }
}

#[catch(401)]
fn unauthorized(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::unauthorized(req.uri().as_str().into()))
}

#[catch(400)]
fn bad_request(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::bad_request(req.uri().as_str().into()))
}

#[catch(404)]
fn not_found(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::not_found(req.uri().as_str().into()))
}

#[catch(500)]
fn internal_server_error(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::internal_server_error(
        req.uri().as_str().into(),
    ))
}

#[catch(503)]
fn service_unavailable(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::service_unavailable(
        req.uri().as_str().into(),
    ))
}

pub fn init_error_handlers() -> Vec<Catcher> {
    catchers![
        unauthorized,
        not_found,
        internal_server_error,
        bad_request,
        service_unavailable
    ]
}