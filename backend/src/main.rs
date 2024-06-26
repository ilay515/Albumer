#[macro_use]
extern crate rocket; 
mod cors;
mod routes;
mod spotify;

use dotenv::dotenv;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set.");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set.");
    let token = match spotify::auth::get_access_token(client_id, client_secret).await {
        Ok(token) => token,
        Err(err) => panic!("Error: {}", err)
    };

    rocket::build()
        .manage(token)
        .attach(cors::create_cors())
        .mount("/library", routes![routes::library::get_library])
        // .mount("/import", routes![routes::import::import_user_albums])
}