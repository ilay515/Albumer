use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Cors, CorsOptions
};

pub fn create_cors() -> Cors {
    let allowed_origins = AllowedOrigins::all();

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error while building CORS");
    cors
}