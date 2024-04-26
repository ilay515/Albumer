use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    url: String,
    height: i32,
    width: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    name: String,
    artists: Vec<Artist>,
    images: Vec<Image>,
    release_date: String,
    genres: Vec<String>,
    popularity: i32
}


