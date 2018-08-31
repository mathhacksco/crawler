use reqwest::header::Accept;
use reqwest::{self, StatusCode};
use serde_json::de::from_str;
use std::collections::HashMap;
use std::io::Read;

use Error;

pub fn fetch_publication(publication: &str) -> Result<MediumPublicationResponse, Error> {
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediumPublicationResponse {
    pub payload: MediumPublicationResponsePayload,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediumPublicationResponsePayload {
    pub references: MediumPublicationResponsePayloadReferences,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediumPublicationResponsePayloadReferences {
    #[serde(rename = "Post")]
    pub post: HashMap<String, MediumPostResponse>,
    #[serde(rename = "User")]
    pub user: HashMap<String, MediumUserResponse>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediumUserResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub name: String,
    pub username: String,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "imageId")]
    pub image_id: String,
    #[serde(rename = "backgroundImageId")]
    pub background_image_id: String,
    pub bio: String,
    #[serde(rename = "twitterScreenName")]
    pub twitter_screen_name: String,
    #[serde(rename = "facebookAccountId")]
    pub facebook_account_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediumPostResponse {
    pub id: String,
    pub title: String,
    pub slug: String,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    #[serde(rename = "uniqueSlug")]
    pub unique_slug: String,
    #[serde(rename = "previewContent")]
    pub preview_content: MediumPostResponsePreviewContent,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediumPostResponsePreviewContent {
    #[serde(rename = "bodyModel")]
    pub body_model: MediumPostResponseBodyModel,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediumPostResponseBodyModel {
    pub paragraphs: Vec<MediumPostResponseBodyModelParagraph>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediumPostResponseBodyModelParagraph {
    pub text: String,
}
