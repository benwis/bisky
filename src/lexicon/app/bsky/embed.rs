use crate::lexicon::com::atproto::repo::{Blob, StrongRef, BlobOutput};
use serde::{Deserialize, Serialize};

use super::feed::ImagesEmbed;

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

///app.bsky.embed.external#external
#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalObject {
    pub uri: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<BlobOutput>,
    pub description: String,
    #[serde(rename(deserialize = "maxSize", serialize = "maxSize"))]
    pub max_size: Option<usize>,
}

///app.bsky.embed.external
#[derive(Debug, Deserialize, Serialize)]
pub struct External {
    pub external: ExternalObject,
}
/// app.bsky.embed.record
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub record: StrongRef
}

/// app.bsky.embed.recordWithMedia
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecordWithMedia {
    pub media: MainMediaEnum,
    pub record: Record,
}

#[allow(clippy::large_enum_variant)]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "$type")]
pub enum MainMediaEnum {
    #[serde(rename = "app.bsky.embed.images")]
    AppBskyEmbedImagesMain(ImagesEmbed),
    #[serde(rename = "app.bsky.embed.external")]
    AppBskyEmbedExternalMain(External),
}