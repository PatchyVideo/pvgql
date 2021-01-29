

use juniper::graphql_value;
use std::collections::BTreeMap;

use juniper::FieldResult;

use crate::common::*;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::{Meta, Error, RestResult, BsonDateTime, Video, VideoItem};
use crate::context::Context;

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="listVideo required parameters", Context = Context)]
pub struct ListVideoParameters {
	/// Offset (start from 0)
	pub offset: i32,
	/// Num of item in a page
	pub limit: i32,
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

#[juniper::graphql_object(Context = Context)]
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


pub async fn listVideo_impl(context: &Context, para: ListVideoParameters) -> FieldResult<ListVideoResult> {
	let result = if para.query.is_none() {
		postJSON!(ListVideoResult, format!("{}/listvideo.do", BACKEND_URL), para, context)
	} else {
		postJSON!(ListVideoResult, format!("{}/queryvideo.do", BACKEND_URL), para, context)
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
