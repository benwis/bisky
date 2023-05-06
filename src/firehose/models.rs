use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::lexicon::app::bsky::{feed::{ImagesEmbed, ReplyRef}, embed::{Record, RecordWithMedia}};

#[derive(Debug, Deserialize, Serialize)]
pub struct FirehosePost {
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: DateTime<Utc>,
    #[serde(rename(deserialize = "$type", serialize = "$type"))]
    pub rust_type: Option<String>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<FirehoseEmbeds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<ReplyRef>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "$type")]
pub enum FirehoseEmbeds {
    #[serde(rename(
        deserialize = "app.bsky.embed.images",
        serialize = "app.bsky.embed.images"
    ))]
    Images(FirehoseImagesEmbed),
    #[serde(rename(
        deserialize = "app.bsky.embed.external",
        serialize = "app.bsky.embed.external"
    ))]
    External(FirehoseExternal),
    #[serde(rename(deserialize = "app.bsky.embed.record"))]
    Record(Record),
    #[serde(rename(deserialize = "app.bsky.embed.recordWithMedia"))]
    RecordWithMedia(FirehoseRecordWithMedia),
}

///app.bsky.embed.external#external
#[derive(Debug, Deserialize, Serialize)]
pub struct FirehoseExternalObject {
    pub uri: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<FirehoseBlob>,
    pub description: String,
    #[serde(rename(deserialize = "maxSize", serialize = "maxSize"))]
    pub max_size: Option<usize>,
}

///app.bsky.embed.external
#[derive(Debug, Deserialize, Serialize)]
pub struct FirehoseExternal {
    pub external: FirehoseExternalObject,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirehoseBlob {
    #[serde(rename(deserialize = "$type", serialize = "$type"))]
    pub rust_type: String,
    #[serde(with = "serde_bytes")]
    pub r#ref: Vec<u8>,
    #[serde(rename(deserialize = "mimeType", serialize = "mimeType"))]
    pub mime_type: String,
    pub size: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FirehoseImage {
    pub image: FirehoseBlob,
    pub alt: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FirehoseImagesEmbed {
    pub images: Vec<FirehoseImage>,
}

/// app.bsky.embed.recordWithMedia
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FirehoseRecordWithMedia {
    pub media: FirehoseMainMediaEnum,
    pub record: Record,
}

#[allow(clippy::large_enum_variant)]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "$type")]
pub enum FirehoseMainMediaEnum {
    #[serde(rename = "app.bsky.embed.images")]
    AppBskyEmbedImagesMain(FirehoseImagesEmbed),
    #[serde(rename = "app.bsky.embed.external")]
    AppBskyEmbedExternalMain(FirehoseExternal),
}