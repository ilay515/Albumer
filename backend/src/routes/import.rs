#[path = "../spotify/api.rs"] mod api;
#[path = "../structs/album.rs"] mod album;
use serde::{Deserialize, Serialize};
use rocket::{serde::json::Json, State};

#[derive(Serialize, Deserialize, Debug)]
struct UserAlbums {
    items: Vec<album::Album>
}

#[get("/")]
pub async fn import_user_albums(token: &State<String>) -> Json<Vec<album::Album>> {
    let data = match api::get_resource(
            token.as_str(), 
            "me/albums", 
        ).await {
        Ok(data) => data,
        Err(err) => panic!("Error: {}", err)
    };
    println!("{:?}", data);

    let albums = match serde_json::from_str::<UserAlbums>(&data) {
        Ok(albums) => albums,
        Err(err) => panic!("Error: {}", err)
    };
    Json(albums.items)
}

