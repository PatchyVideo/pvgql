

use juniper::graphql_value;
use juniper::GraphQLObject;

use juniper::FieldResult;

use crate::common::*;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use crate::models::*;
use std::convert::{TryFrom, TryInto};
use crate::models::{Meta, Error, RestResult, BsonDateTime, Video, VideoItem};
use crate::context::Context;

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="listVideo required parameters", Context = Context)]
pub struct ListVideoParameters {
	/// Offset (start from 0)
	pub offset: Option<i32>,
	/// Num of item in a page
	pub limit: Option<i32>,
	/// Query
	pub query: Option<String>,
	/// Query type, one of tag, text
	pub qtype: Option<String>,
	/// List order, one of 'latest', 'oldest', 'video_latest', 'video_oldest', 'last_modified'
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
	pub page_count: i32,
	pub related_tagids: Option<Vec<i64>>,
	pub tagid_popmap: Option<serde_json::Map<String, serde_json::Value>>
}

#[juniper::graphql_object(Context = Context)]
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
	pub async fn related_tags(&self, context: &Context) -> FieldResult<Option<Vec<TagObjectValue>>> {
		if let Some(tagids) = self.related_tagids.as_ref() {
			Ok(Some(super::editTags::getTagObjectsBatch_impl(context, super::editTags::GetTagObjectsBatchParameters {
				tagid: tagids.iter().filter(|&n| { *n < 2_147_483_647i64 }).map(|&n| n as i32).collect::<Vec<_>>()
			}).await?))
		} else {
			Ok(None)
		}
	}
	pub async fn popular_tags(&self, context: &Context) -> FieldResult<Option<Vec<TagWithPopularity>>> {
		// if let Some(tagid_maps) = self.tagid_popmap.as_ref() {
		// 	let tagids = tagid_maps.keys().map(|k| k.parse::<i64>().unwrap()).collect::<Vec<_>>();
		// 	let mut tagobjs: Vec<TagObjectValue> = super::editTags::getTagObjectsBatch_impl(context, super::editTags::GetTagObjectsBatchParameters {
		// 		tagid: tagids.iter().filter(|&n| { *n < 2_147_483_647i64 }).map(|&n| n as i32).collect::<Vec<_>>()
		// 	}).await?;
		// 	Ok(Some(tagid_maps.values().zip(tagobjs.iter_mut()).map(|(k, v)| {
		// 		TagWithPopularity {
		// 			popluarity: k.as_i64().unwrap() as _,
		// 			tag: *v
		// 		}
		// 	}).collect::<Vec<_>>()))
		// } else {
		// 	Ok(None)
		// }
		Ok(None)
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

