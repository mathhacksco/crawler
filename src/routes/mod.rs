use redis::Commands;
use rocket::http::uri::URI;
use rocket::http::Status;
use rocket::response::Response;
use rocket::{Catcher, Request, Route, State};
use rocket_contrib::Json;
use std::sync::{Arc, Mutex};

use integrations::medium::api::fetch_publication;
use models::{Post, User};
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
            let posts = serde_json::from_str::<Vec<Post>>(&*posts_str)?;
            Ok(Json(PostsPayload {
                posts
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
            let posts = get_medium_posts()?;
            redis_conn.set("@mathhacks:medium_posts", serde_json::to_string(&posts)?)?;
            Ok(Response::build().status(Status::NoContent).finalize())
        }
        Err(_) => Err(Error::RouteError(ErrorResponse::service_unavailable(
            uri.as_str().into(),
        ))),
    }
}

fn get_medium_posts() -> Result<Vec<Post>, Error> {
    let res = fetch_publication(MEDIUM_PUBLICATION)?;
    let users = res.payload.references.user;
    let medium_posts = res.payload.references.post;
    let posts = medium_posts
        .iter()
        .map(|(_, p)| {
            let author = users.get(&*p.creator_id).unwrap().clone();
            Post {
                id: p.id.clone(),
                slug: p.unique_slug.clone(),
                title: p.title.clone(),
                author: User {
                    id: author.user_id,
                    name: author.name,
                    username: author.username,
                    image_id: author.image_id,
                    background_image_id: author.background_image_id,
                    bio: author.bio,
                    twitter_screen_name: author.twitter_screen_name,
                    facebook_account_id: author.facebook_account_id,
                    created_at: author.created_at,
                },
                subtitles: p
                    .preview_content
                    .body_model
                    .paragraphs
                    .iter()
                    .map(|p| p.text.clone())
                    .collect(),
                created_at: p.created_at,
                updated_at: p.updated_at,
            }
        }).collect::<Vec<Post>>();
    Ok(posts)
}

#[catch(401)]
fn unauthorized(req: &Request) -> ErrorResponse {
    ErrorResponse::unauthorized(req.uri().as_str().into())
}

#[catch(400)]
fn bad_request(req: &Request) -> ErrorResponse {
    ErrorResponse::bad_request(req.uri().as_str().into())
}

#[catch(404)]
fn not_found(req: &Request) -> ErrorResponse {
    ErrorResponse::not_found(req.uri().as_str().into())
}

#[catch(500)]
fn internal_server_error(req: &Request) -> ErrorResponse {
    ErrorResponse::internal_server_error(req.uri().as_str().into())
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
