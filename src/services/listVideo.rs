

use juniper::graphql_value;
use std::collections::BTreeMap;

use juniper::FieldResult;

use crate::common::*;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;

use crate::models::{Meta, Error, RestResult};

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


#[derive(Clone, Serialize, Deserialize)]
pub struct VideoItem {
    pub cover_image: String,
    pub title: String,
    pub desc: String,
    pub placeholder: bool,
    pub rating: f64,
    pub repost_type: String,
    pub copies: Vec<ObjectId>,
    pub series: Vec<ObjectId>,
    pub site: String,
    pub thumbnail_url: String,
    pub unique_id: String,
    //pub upload_time: bson::DateTime,
    pub url: String,
    pub user_space_urls: Option<Vec<String>>,
    pub utags: Vec<String>,
    pub views: i32
}

#[juniper::object]
#[graphql(description="Video Item")]
impl VideoItem {
    pub fn cover_image(&self) -> &String {
        &self.cover_image
    }
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn desc(&self) -> &String {
        &self.desc
    }
    pub fn placeholder(&self) -> &bool {
        &self.placeholder
    }
    pub fn rating(&self) -> &f64 {
        &self.rating
    }
    pub fn repost_type(&self) -> &String {
        &self.repost_type
    }
    // pub fn copies(&self) -> &Vec<Video> {
    //     //
    // }
    // pub fn series(&self) -> &Vec<ObjectId> {
    //     &self.series
    // }
    pub fn site(&self) -> &String {
        &self.site
    }
    pub fn thumbnail_url(&self) -> &String {
        &self.thumbnail_url
    }
    pub fn unique_id(&self) -> &String {
        &self.unique_id
    }
    // pub fn upload_time(&self) -> &DateTime<chrono::Utc> {
    //     &self.upload_time
    // }
    pub fn url(&self) -> &String {
        &self.url
    }
    pub fn user_space_urls(&self) -> &Option<Vec<String>> {
        &self.user_space_urls
    }
    pub fn utags(&self) -> &Vec<String> {
        &self.utags
    }
    pub fn views(&self) -> &i32 {
        &self.views
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Video {
    pub clearence: i32,
    pub item: VideoItem,
    pub meta: Meta,
    pub tag_count: i32,
    pub tags: Vec<i32>,
    pub tags_readable: Option<Vec<String>>
}

#[juniper::object]
#[graphql(description="Video Item")]
impl Video {
    pub fn clearence(&self) -> &i32 {
        &self.clearence
    }
    pub fn item(&self) -> &VideoItem {
        &self.item
    }
    pub fn meta(&self) -> &Meta {
        &self.meta
    }
    pub fn tag_count(&self) -> &i32 {
        &self.tag_count
    }
    pub fn tags(&self) -> &Vec<i32> {
        &self.tags
    }
    pub fn tags_readable(&self) -> &Option<Vec<String>> {
        &self.tags_readable
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListVideoResult {
    pub videos: Vec<Video>,
    pub count: i32,
    pub page_count: i32
}

#[juniper::object]
#[graphql(description="List video result")]
impl ListVideoResult {
    pub fn videos(&self) -> &Vec<Video> {
        &self.videos
    }
    pub fn count(&self) -> &i32 {
        &self.count
    }
    pub fn page_count(&self) -> &i32 {
        &self.page_count
    }
}


pub fn listVideo_impl(para: ListVideoParameters) -> FieldResult<ListVideoResult> {
    let result = postJSON!(ListVideoResult, format!("https://thvideo.tv/be/listvideo.do"), para);
    if result.status == "SUCCEED" {
        Ok(result.data.unwrap())
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
