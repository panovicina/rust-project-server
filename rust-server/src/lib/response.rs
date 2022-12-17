use serde;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "response_type")]
#[serde(rename_all = "snake_case")]
pub enum Response {
    Success {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        data: Option<DataRespose>,
    },
    KeyNotFound,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DataRespose {
    key: String,
    hash: String,
}

impl DataRespose {
    pub fn new(key: String, hash: String) -> Self {
        DataRespose { key, hash }
    }
}
