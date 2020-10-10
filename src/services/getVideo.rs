
use juniper::graphql_value;
use std::collections::BTreeMap;

use juniper::FieldResult;

use crate::common::*;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::*;

#[derive(Serialize, Deserialize)]
pub struct GetVideoResponse {
    pub video: Video,
    pub tag_by_category: serde_json::Map<String, serde_json::Value>,
    pub playlists: Vec<PlaylistContentForVideo>,
    pub copies: Vec<Video>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="listVideo required parameters")]
pub struct GetVideoParameters {
    /// Video ID
    pub vid: String,
    /// Language
    pub lang: String
}

pub async fn getVideo_impl(para: GetVideoParameters) -> FieldResult<Video> {
    let result = postJSON!(GetVideoResponse, format!("https://thvideo.tv/be/getvideo.do"), para);
    if result.status == "SUCCEED" {
        let resp = result.data.unwrap();
        let mut video = resp.video;
        video.copies = Some(resp.copies);
        video.playlists = Some(resp.playlists);
        let mut catemap: Vec<TagCategoryItem> = vec![];
        for (k, v) in resp.tag_by_category {
            catemap.push(TagCategoryItem {
                key: k,
                value: v.as_array().unwrap().iter().map(|x: &serde_json::Value| x.as_str().unwrap().into()).collect::<Vec<_>>()
            });
        };
        video.tag_by_category = Some(catemap);
        Ok(video)
    } else {
        Err(
            juniper::FieldError::new(
                result.status,
                graphql_value!({
                    "aa"
                }),
            )
        )
    }
}
