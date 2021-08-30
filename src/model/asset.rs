use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum Metadata {
    File,
    Text,
    Image { width: isize, height: isize },
    Video { width: isize, height: isize },
    Audio,
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    #[serde(rename = "_id")]
    id: String,
    tag: String,
    filename: String,
    metadata: Metadata,
    content_type: String,
    size: isize,
    deleted: Option<bool>,
    message_id: Option<String>,
    user_id: Option<String>,
    server_id: Option<String>,
    object_id: Option<String>,
}