use juniper::{Value, graphql_value};


use juniper::FieldResult;

use crate::{common::*, models::{Playlist, TagCategoryItem}};

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::{Meta, Error, RestResult, Video, PlaylistMeta};
use crate::context::Context;

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="PostVideo data", Context = Context)]
pub struct PostVideoRequestData {
	/// Video URL
	pub url: String,
	/// Video tags
	pub tags: Vec<String>,
	/// Reference to another copy, in video ObjectId format
	pub copy: Option<String>,
	/// Playlist ID if you want to add this video to a playlist
	pub pid: Option<String>,
	/// Rank of video in the playlist you are inserting into, default to last position
	pub rank: Option<i32>,
	/// Type of repost, one of 'official', 'official_repost', 'authorized_translation', 'authorized_repost', 'translation', 'repost', 'unknown'
	pub repost_type: Option<String>,
	/// Behaviour of tags if this video already exists, one of 'merge', 'keep_existing', default 'merge'
	pub tag_merge_behaviour: Option<String>,
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="BatchPostVideo data", Context = Context)]
pub struct BatchPostVideoRequestData {
	/// Video URLs
	pub videos: Vec<String>,
	/// Video tags
	pub tags: Vec<String>,
	/// Reference to another copy, in video ObjectId format
	pub copy: Option<String>,
	/// Playlist ID if you want to add this video to a playlist
	pub pid: Option<String>,
	/// Rank of video in the playlist you are inserting into, default to last position
	pub rank: Option<i32>,
	/// Type of repost, one of 'official', 'official_repost', 'authorized_translation', 'authorized_repost', 'translation', 'repost', 'unknown'
	pub repost_type: Option<String>,
	/// If we should treat videos as copies to each other
	pub as_copies: Option<bool>,
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct PostVideoResult {
	pub task_id: String
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct BatchPostVideoResult {
	pub task_ids: String
}

pub async fn postVideo_impl(context: &Context, para: PostVideoRequestData) -> FieldResult<PostVideoResult> {
	let result = postJSON!(PostVideoResult, format!("{}/postvideo.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		let r = result.data.unwrap();
		Ok(r)
	} else {
		let r = result.dataerr.unwrap();
		Err(
			juniper::FieldError::new(
				r.reason,
				r.aux.map_or(Value::Null, |f| graphql_value!({
					f
				})),
			)
		)
	}
}

pub async fn batchPostVideo_impl(context: &Context, para: BatchPostVideoRequestData) -> FieldResult<BatchPostVideoResult> {
	let result = postJSON!(BatchPostVideoResult, format!("{}/postvideo_batch.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		let r = result.data.unwrap();
		Ok(r)
	} else {
		let r = result.dataerr.unwrap();
		Err(
			juniper::FieldError::new(
				r.reason,
				r.aux.map_or(Value::Null, |f| graphql_value!({
					f
				})),
			)
		)
	}
}
