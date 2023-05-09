use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StrongRef {
    pub uri: String,
    pub cid: String,
}

#[derive(Debug, Deserialize)]
pub struct Record<T> {
    pub uri: String,
    pub cid: String,
    pub value: T,
}

#[derive(Debug, Deserialize)]
pub struct ListRecordsOutput<T> {
    pub cursor: Option<String>,
    pub records: Vec<Record<T>>,
}

#[derive(Serialize)]
pub struct CreateRecord<'a, T> {
    pub repo: &'a str,
    pub collection: &'a str,
    pub record: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rkey: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "swapCommit"))]
    pub swap_commit: Option<&'a str>

}

#[derive(Serialize)]
pub struct PutRecord<'a, T> {
    pub repo: &'a str,
    pub collection: &'a str,
    pub record: T,
    pub rkey: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "swapRecord"))]
    pub swap_record: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "swapCommit"))]
    pub swap_commit: Option<&'a str>

}

#[derive(Serialize)]
pub struct DeleteRecord<'a> {
    pub repo: &'a str,
    pub collection: &'a str,
    pub rkey: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "swapRecord"))]
    pub swap_record: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "swapCommit"))]
    pub swap_commit: Option<&'a str>

}

#[derive(Debug, Deserialize)]
pub struct CreateRecordOutput {
    pub cid: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUploadBlob {
    pub blob: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename(deserialize = "$link", serialize = "$link"))]
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blob {
    #[serde(rename(deserialize = "$type", serialize = "$type"))]
    pub rust_type: String,
    //#[serde(with = "serde_bytes")]
    pub r#ref: Link,
    #[serde(rename(deserialize = "mimeType", serialize = "mimeType"))]
    pub mime_type: String,
    pub size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlobOutput {
    pub blob: Blob,
}
