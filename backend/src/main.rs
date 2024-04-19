#[macro_use]
extern crate rocket; 
mod cors;
mod albums;
mod spotify_api;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::create_cors())
        .mount("/albums", routes![albums::get_all_albums])
}