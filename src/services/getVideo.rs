
use juniper::graphql_value;


use juniper::FieldResult;

use crate::common::*;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::*;
use crate::context::Context;

#[derive(Serialize, Deserialize)]
pub struct GetVideoResponse {
	pub video: Video,
	pub tag_by_category: serde_json::Map<String, serde_json::Value>,
	pub playlists: Vec<PlaylistContentForVideo>,
	pub copies: Vec<Video>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="listVideo required parameters", Context = Context)]
pub struct GetVideoParameters {
	/// Video ID
	pub vid: String,
	/// Language
	pub lang: String
}

pub async fn getVideo_impl(context: &Context, para: GetVideoParameters) -> FieldResult<Video> {
	let result = postJSON!(GetVideoResponse, format!("{}/getvideo.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		let resp = result.data.unwrap();
		let mut video = resp.video;
		video.copies = Some(resp.copies);
		video.playlists = Some(resp.playlists);
		let mut catemap: Vec<TagCategoryItem> = vec![];
		for (k, v) in resp.tag_by_category {
			catemap.push(TagCategoryItem {
				key: TagCategoryEnum::from_string(&k)?,
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

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="GetRelatedVideo required parameters", Context = Context)]
pub struct GetRelatedVideoParameters {
	pub vid: String,
	pub top_k: Option<i32>,
	pub sort_title: Option<bool>
}

#[derive(Serialize, Deserialize)]
pub struct GetRelatedVideoPesponse {
	pub videos: Vec<Video>
}

pub async fn getRelatedVideo_impl(context: &Context, para: GetRelatedVideoParameters) -> FieldResult<Vec<Video>> {
	let result = postJSON!(GetRelatedVideoPesponse, format!("{}/get_related_videos.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		Ok(result.data.unwrap().videos)
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

#[test]
fn untyped_example() -> Result<(), Box<dyn std::error::Error>> {
	use serde_json;
	// Some JSON input data as a &str. Maybe this comes from the user.
	let data = r#"
		{
			"name": "",
			"age": 43,
			"phones": [
				"+44 1234567",
				"+44 2345678"
			]
		}"#;

	// Parse the string of data into serde_json::Value.
	let v: serde_json::Value = serde_json::from_str(data)?;

	// Access parts of the data by indexing with square brackets.
	println!("Please call {} at the number {}", v["name"], v["phones"][0]);

	Ok(())
}
