
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use bson::DateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
    pub code: String,
    pub aux: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestResult<T> {
    pub status: String,
    pub data: Option<T>
}

#[derive(Serialize, Deserialize)]
pub struct MetaRest {

}


#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct Meta {

}

// #[derive(Serialize, Deserialize)]
// pub struct VideoItemRest {
//     pub cover_image: String,
//     pub title: String,
//     pub desc: String,
//     pub placeholder: bool,
//     pub rating: i32,
//     pub repost_type: String,
//     pub copies: Vec<ObjectId>,
//     pub series: Vec<ObjectId>,
//     pub site: String,
//     pub thumbnail_url: String,
//     pub unique_id: String,
//     pub upload_time: DateTime,
//     pub url: String,
//     pub user_space_urls: Option<Vec<String>>,
//     pub utags: Vec<String>,
//     pub views: i32
// }

// #[derive(Serialize, Deserialize)]
// pub struct VideoRest {
//     pub clearence: i32,
//     pub item: VideoItemRest,
//     pub meta: MetaRest,
//     pub tag_count: i32,
//     pub tags: Vec<i32>,
//     pub tags_readable: Option<Vec<String>>
// }
