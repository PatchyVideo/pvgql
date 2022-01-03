
use juniper::graphql_value;


use juniper::{FieldResult, ScalarValue};

use crate::common::*;
use crate::services::authorDB;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::*;
use crate::context::Context;
use super::editTags;

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="editVideoTags required parameters", Context = Context)]
pub struct EditVideoTagsParameters {
	/// Video ID
	pub video_id: String,
	/// Tags
	pub tags: Vec<String>,
	/// One of 'replace', 'append', 'remove'
	pub edit_behaviour: String,
	/// Behaviour if a tag does not exist, one of 'ignore', 'error', 'append', default 'ignore'
	pub not_found_behaviour: Option<String>,
	/// User language used for adding tags, default is 'ENG'
	pub user_language: Option<String>,
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="editVideoTagIds required parameters", Context = Context)]
pub struct EditVideoTagIdsParameters {
	/// Video ID
	pub video_id: String,
	/// Tags
	pub tags: Vec<i32>,
	/// One of 'replace', 'append', 'remove'
	pub edit_behaviour: String,
	/// Behaviour if a tag does not exist, one of 'ignore', 'error', 'append', default 'ignore'
	pub not_found_behaviour: Option<String>,
	/// User language used for adding tags, default is 'ENG'
	pub user_language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditVideoTagsRespObject {
	pub tagids: Vec<i32>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="SetVideoClearence required parameters", Context = Context)]
pub struct SetVideoClearenceParameters {
	/// Video ID
	pub vid: String,
	/// Clearence, one of 0, 1, 2, 3, default is 0
	pub clearence: Option<i32>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetVideoClearenceRespObject {
	pub clearence: i32
}

pub async fn editVideoTags_impl(context: &Context, para: EditVideoTagsParameters) -> FieldResult<Vec<TagObjectValue>> {
	let result = postJSON!(EditVideoTagsRespObject, format!("{}/videos/edittags.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		let tagids = result.data.unwrap().tagids;
		editTags::getTagObjectsBatch_impl(context, editTags::GetTagObjectsBatchParameters {
			tagid: tagids
		}).await
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

pub async fn editVideoTagIds_impl(context: &Context, para: EditVideoTagIdsParameters) -> FieldResult<Vec<TagObjectValue>> {
	let result = postJSON!(EditVideoTagsRespObject, format!("{}/videos/edittagids.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		let tagids = result.data.unwrap().tagids;
		editTags::getTagObjectsBatch_impl(context, editTags::GetTagObjectsBatchParameters {
			tagid: tagids
		}).await
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

pub async fn setVideoClearenceVideo_impl(context: &Context, para: SetVideoClearenceParameters) -> FieldResult<i32> {
	let result = postJSON!(SetVideoClearenceRespObject, format!("{}/videos/set_clearence.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		Ok(result.data.unwrap().clearence)
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

