use crate::lexicon::com::atproto::repo::Blob;
use serde::{Deserialize, Serialize};

// "app.bsky.embed.images#view",
// "app.bsky.embed.external#view",
// "app.bsky.embed.record#view",
// "app.bsky.embed.recordWithMedia#view"

///app.bsky.embed.images
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Images {
//     pub images: Vec<Image>,
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub image: Blob,
    pub alt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ViewImage {
    pub thumb: String,
    #[serde(rename(deserialize = "fullSize", serialize = "fullSize"))]
    pub full_size: String,
    pub alt: String,
}
