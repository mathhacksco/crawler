#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub author: User,
    pub subtitles: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub image_id: String,
    pub background_image_id: String,
    pub bio: String,
    pub twitter_screen_name: String,
    pub facebook_account_id: String,
    pub created_at: u64,
}
