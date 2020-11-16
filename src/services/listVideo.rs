

use juniper::graphql_value;
use std::collections::BTreeMap;

use juniper::FieldResult;

use crate::common::*;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::{Meta, Error, RestResult, BsonDateTime, Video, VideoItem};

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="listVideo required parameters")]
pub struct ListVideoParameters {
	/// Page Number (starts from 1)
	pub page: i32,
	/// Num of item in a page
	pub page_size: i32,
	/// Query
	pub query: Option<String>,
	/// Query type, one of tag, text
	pub qtype: Option<String>,
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
pub struct ListVideoResult {
	pub videos: Vec<Video>,
	pub count: i32,
	pub page_count: i32
}

#[juniper::graphql_object]
#[juniper::graphql(description="List video result")]
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


pub async fn listVideo_impl(para: ListVideoParameters) -> FieldResult<ListVideoResult> {
	let result = if para.query.is_none() {
		postJSON!(ListVideoResult, format!("https://thvideo.tv/be/listvideo.do"), para)
	} else {
		postJSON!(ListVideoResult, format!("https://thvideo.tv/be/queryvideo.do"), para)
	};
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
