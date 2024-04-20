#[path = "../spotify/api.rs"] mod api;
#[path = "../structs/album.rs"] mod album;
use serde::{Deserialize, Serialize};
use rocket::{serde::json::Json, State};

#[derive(Serialize, Deserialize, Debug)]
struct Albums {
    albums: Vec<album::Album>
}

#[get("/")]
pub async fn get_library(token: &State<String>) -> Json<Vec<album::Album>> {
    let album_ids = "7caGY3YPOchIO8xLvTKWN4,5zi7WsKlIiUXv09tbGLKsE,382ObEPsp2rxGrnsizN5TX,1A2GTWGtFfWp7KSQTwWOyo,2noRn2Aes5aoNVsU6iWThc";
    let data = match api::get_resource_by_ids(
            token.as_str(), 
            "albums",
            album_ids
        ).await {
        Ok(data) => data,
        Err(err) => panic!("Error: {}", err)
    };

    let albums = match serde_json::from_str::<Albums>(&data) {
        Ok(albums) => albums,
        Err(err) => panic!("Error: {}", err)
    };
    Json(albums.albums)
}
