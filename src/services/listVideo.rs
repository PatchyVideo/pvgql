

use juniper::graphql_value;
use std::collections::BTreeMap;

use juniper::FieldResult;

use crate::common::*;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;

use crate::models::Meta;

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="listVideo required parameters")]
pub struct ListVideoParameters {
    /// Page Number (starts from 1)
    pub page: i32,
    /// Num of item in a page
    pub page_size: i32,
    /// Query
    pub query: Option<String>,
    /// List order, one of 'latest', 'oldest', 'video_latest', 'video_oldest'
    pub order: Option<String>,
    // Addtional query constraints
    pub additional_constraint: Option<String>,
    /// If true, no placeholder items will be shown
    pub hide_placeholder: Option<bool>,
    /// User language
    pub lang: Option<String>,
    /// Add tags_readable field to every result item
    pub human_readable_tag: Option<bool>
}


#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct VideoItem {
    pub cover_image: String,
    pub title: String,
    pub desc: String,
    pub placeholder: bool,
    pub rating: i32,
    pub repost_type: String,
    pub copies: Vec<ObjectId>,
    pub series: Vec<ObjectId>,
    pub site: String,
    pub thumbnail_url: String,
    pub unique_id: String,
    pub upload_time: bson::DateTime,
    pub url: String,
    pub user_space_urls: Option<Vec<String>>,
    pub utags: Vec<String>,
    pub views: i32
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct Video {
    pub clearence: i32,
    pub item: VideoItem,
    pub meta: Meta,
    pub tag_count: i32,
    pub tags: Vec<i32>,
    pub tags_readable: Option<Vec<String>>
}


#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="listVideo result")]
pub struct ListVideoResult {
    pub videos: Vec<Video>,
    pub count: i32,
    pub page_count: i32
}

pub fn listVideo_impl(para: ListVideoParameters) -> FieldResult<ListVideoResult> {
    
}
