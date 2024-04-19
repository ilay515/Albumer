#[path = "spotify_api.rs"] mod spotify_api;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Image {
    url: String,
    height: i32,
    width: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    images: Vec<Image>,
    release_date: String,
    genres: Vec<String>,
    popularity: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Albums {
    albums: Vec<Album>
}

#[get("/")]
pub async fn get_all_albums() -> Json<Vec<Album>> {
    let album_ids = "7caGY3YPOchIO8xLvTKWN4,5zi7WsKlIiUXv09tbGLKsE,382ObEPsp2rxGrnsizN5TX,1A2GTWGtFfWp7KSQTwWOyo,2noRn2Aes5aoNVsU6iWThc";
    let data = match spotify_api::get_albums(album_ids).await {
        Ok(data) => data,
        Err(err) => panic!("Error: {}", err)
    };

    let albums = match serde_json::from_str::<Albums>(&data) {
        Ok(albums) => albums,
        Err(err) => panic!("Error: {}", err)
    };
    Json(albums.albums)
}

