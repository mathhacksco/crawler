use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};
use std::sync::{Arc, Mutex};

use routes::{init_error_handlers, init_routes};

pub fn init_server(redis_conn: redis::Connection) {
    rocket::ignite()
        .manage(Arc::new(Mutex::new(redis_conn)))
        .manage(cors_options())
        .mount("/", rocket_cors::catch_all_options_routes())
        .mount("/", init_routes())
        .catch(init_error_handlers())
        .attach(cors_options())
        .launch();
}

fn cors_options() -> Cors {
    let origins = dotenv!("ALLOWED_ORIGINS").split(",").collect::<Vec<&str>>();
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&origins);
    assert!(failed_origins.is_empty());
    rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
}
