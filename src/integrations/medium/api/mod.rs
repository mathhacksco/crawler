use reqwest::header::Accept;
use reqwest::{self, StatusCode};
use serde_json::de::from_str;
use std::collections::HashMap;
use std::io::Read;

use Error;

#[derive(Deserialize, Debug, Clone)]
pub struct MediumPublicationResponse {
    pub payload: MediumPublicationResponsePayload,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediumPublicationResponsePayload {
    pub references: MediumPublicationResponsePayloadReferences,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediumPublicationResponsePayloadReferences {
    #[serde(rename = "Post")]
    pub post: HashMap<String, MediumPostResponse>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediumPostResponse {
    pub id: String,
    pub title: String,
}

pub fn fetch_posts(publication: String) -> Result<MediumPublicationResponse, Error> {
    let client = reqwest::Client::new();
    let url = format!("https://medium.com/{}", publication);
    let mut res = client.get(&url).header(Accept::json()).send()?;
    match res.status() {
        StatusCode::Ok => {
            let mut content = String::new();
            res.read_to_string(&mut content).unwrap();
            let trim_content = content.trim_left_matches("])}while(1);</x>");
            Ok(from_str::<MediumPublicationResponse>(&*trim_content)?)
        }
        _ => {
            let mut content = String::new();
            res.read_to_string(&mut content).unwrap();
            Err(Error::StringError(content))
        }
    }
}
